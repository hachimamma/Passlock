package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
	"time"
)

type PasswordHistory struct {
	Password  string `json:"password"`
	ChangedAt uint64 `json:"changed_at"`
}

type Entry struct {
	ID           string            `json:"id"`
	N            string            `json:"n"`
	U            string            `json:"u"`
	P            string            `json:"p"`
	Url          string            `json:"url,omitempty"`
	Nt           string            `json:"nt,omitempty"`
	T            uint64            `json:"t"`
	Tags         []string          `json:"tags,omitempty"`
	History      []PasswordHistory `json:"history,omitempty"`
	LastModified uint64            `json:"last_modified,omitempty"`
}

type Vault struct {
	E []Entry `json:"e"`
	S string  `json:"s"`
}

var v *Vault
var ms_pwd string

func findPasslockBinary() string {
	if path, err := exec.LookPath("passlock"); err == nil {
		return path
	}

	localPaths := []string{
		"./target/release/passlock",
		"./target/debug/passlock",
		"../target/release/passlock",
	}

	for _, path := range localPaths {
		if _, err := os.Stat(path); err == nil {
			return path
		}
	}

	return "passlock"
}

func calc_pwdS(password string) map[string]interface{} {
	score := 0
	feedback := []string{}

	length := len(password)

	if length >= 8 {
		score += 1
	}
	if length >= 12 {
		score += 1
	}
	if length >= 16 {
		score += 1
	}
	if length < 8 {
		feedback = append(feedback, "Use at least 8 characters")
	}

	hasLower := regexp.MustCompile(`[a-z]`).MatchString(password)
	hasUpper := regexp.MustCompile(`[A-Z]`).MatchString(password)
	hasDigit := regexp.MustCompile(`[0-9]`).MatchString(password)
	hasSpecial := regexp.MustCompile(`[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]`).MatchString(password)

	varietyCount := 0
	if hasLower {
		varietyCount++
	} else {
		feedback = append(feedback, "Add lowercase letters")
	}

	if hasUpper {
		varietyCount++
	} else {
		feedback = append(feedback, "Add uppercase letters")
	}

	if hasDigit {
		varietyCount++
	} else {
		feedback = append(feedback, "Add numbers")
	}

	if hasSpecial {
		varietyCount++
	} else {
		feedback = append(feedback, "Add special characters")
	}

	score += varietyCount

	regex := []string{"123", "abc", "password", "qwerty", "admin"}
	lower_pwd := strings.ToLower(password)
	for _, pattern := range regex {
		if strings.Contains(lower_pwd, pattern) {
			score -= 2
			feedback = append(feedback, "Avoid common patterns")
			break
		}
	}

	if length >= 20 {
		score += 1
	}

	var strength string
	var color string
	var percentage int

	if score < 0 {
		score = 0
	}

	if score <= 2 {
		strength = "Weak"
		color = "red"
		percentage = 25
	} else if score <= 4 {
		strength = "Fair"
		color = "orange"
		percentage = 50
	} else if score <= 6 {
		strength = "Good"
		color = "yellow"
		percentage = 75
	} else {
		strength = "Strong"
		color = "green"
		percentage = 100
	}

	return map[string]interface{}{
		"score":      score,
		"strength":   strength,
		"color":      color,
		"percentage": percentage,
		"feedback":   feedback,
	}
}

func handle(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Access-Control-Allow-Origin", "*")

	if r.Method == "OPTIONS" {
		w.WriteHeader(http.StatusOK)
		return
	}

	if r.Method != "POST" {
		json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "use POST"})
		return
	}

	var req map[string]interface{}
	json.NewDecoder(r.Body).Decode(&req)

	home, _ := os.UserHomeDir()
	vt_path := filepath.Join(home, ".passlock.vault")
	tempP := filepath.Join(home, ".passlock.temp")

	passlockBin := findPasslockBinary()

	act, _ := req["act"].(string)

	switch act {
	case "check":
		exists := false
		if _, err := os.Stat(vt_path); err == nil {
			exists = true
		}
		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "exists": exists})

	case "strength":
		password, _ := req["password"].(string)
		result := calc_pwdS(password)
		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "data": result})

	case "create":
		pwd, _ := req["pwd"].(string)
		confirm, _ := req["confirm"].(string)

		if pwd == "" || confirm == "" {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "password and confirmation required"})
			return
		}

		if pwd != confirm {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "passwords don't match"})
			return
		}

		if len(pwd) < 4 {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "password too short (min 4 chars)"})
			return
		}

		if _, err := os.Stat(vt_path); err == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "vault already exists"})
			return
		}

		cmd := exec.Command(passlockBin, "create", pwd)
		output, err := cmd.CombinedOutput()

		if err != nil {
			json.NewEncoder(w).Encode(map[string]interface{}{
				"ok":  false,
				"msg": fmt.Sprintf("failed to create vault: %s", string(output)),
			})
			return
		}

		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "msg": "vault created successfully"})

	case "unlock":
		pwd, _ := req["pwd"].(string)
		if pwd == "" {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "password required"})
			return
		}

		if _, err := os.Stat(vt_path); os.IsNotExist(err) {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "no vault - create with CLI"})
			return
		}

		cmd := exec.Command(passlockBin, "unlock", pwd)
		output, err := cmd.CombinedOutput()

		if err != nil {
			json.NewEncoder(w).Encode(map[string]interface{}{
				"ok":  false,
				"msg": "wrong password",
			})
			return
		}

		tempD, err := os.ReadFile(tempP)
		if err != nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "failed to read decrypted vault"})
			return
		}

		var tempV Vault
		if err := json.Unmarshal(tempD, &tempV); err != nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "failed to parse vault"})
			return
		}

		ms_pwd = pwd
		v = &tempV
		json.NewEncoder(w).Encode(map[string]interface{}{
			"ok":  true,
			"msg": fmt.Sprintf("unlocked - %s", string(output)),
		})

	case "list":
		if v == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "not unlocked"})
			return
		}
		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "data": v.E})

	case "tags":
		if v == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "not unlocked"})
			return
		}

		tagMap := make(map[string]int)
		for _, entry := range v.E {
			for _, tag := range entry.Tags {
				tagMap[tag]++
			}
		}

		type TagCount struct {
			Tag   string `json:"tag"`
			Count int    `json:"count"`
		}

		tags := []TagCount{}
		for tag, count := range tagMap {
			tags = append(tags, TagCount{Tag: tag, Count: count})
		}

		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "data": tags})

	case "filter":
		if v == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "not unlocked"})
			return
		}

		tag, _ := req["tag"].(string)
		if tag == "" {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "data": v.E})
			return
		}

		filtered := []Entry{}
		for _, entry := range v.E {
			for _, t := range entry.Tags {
				if t == tag {
					filtered = append(filtered, entry)
					break
				}
			}
		}

		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "data": filtered})

	case "add":
		if v == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "not unlocked"})
			return
		}

		name, _ := req["name"].(string)
		user, _ := req["user"].(string)
		pass, _ := req["pass"].(string)
		url, _ := req["url"].(string)
		note, _ := req["note"].(string)

		var tags []string
		if tagsInterface, ok := req["tags"].([]interface{}); ok {
			for _, t := range tagsInterface {
				if tagStr, ok := t.(string); ok {
					tags = append(tags, tagStr)
				}
			}
		}

		if name == "" || user == "" || pass == "" {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "name, user, pass required"})
			return
		}

		now := uint64(time.Now().Unix())
		e := Entry{
			ID:           fmt.Sprintf("%d", time.Now().UnixNano()),
			N:            name,
			U:            user,
			P:            pass,
			Url:          url,
			Nt:           note,
			T:            now,
			Tags:         tags,
			History:      []PasswordHistory{},
			LastModified: now,
		}

		v.E = append(v.E, e)

		tempD, _ := json.Marshal(v)
		os.WriteFile(tempP, tempD, 0600)

		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "msg": "added"})

	case "edit":
		if v == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "not unlocked"})
			return
		}

		id, _ := req["id"].(string)
		name, _ := req["name"].(string)
		user, _ := req["user"].(string)
		pass, _ := req["pass"].(string)
		url, _ := req["url"].(string)
		note, _ := req["note"].(string)

		var tags []string
		if tagsInterface, ok := req["tags"].([]interface{}); ok {
			for _, t := range tagsInterface {
				if tagStr, ok := t.(string); ok {
					tags = append(tags, tagStr)
				}
			}
		}

		found := false
		for i, entry := range v.E {
			if entry.ID == id {
				oldPass := entry.P
				now := uint64(time.Now().Unix())

				if oldPass != pass && pass != "" {
					if len(entry.History) >= 5 {
						entry.History = entry.History[1:]
					}
					entry.History = append(entry.History, PasswordHistory{
						Password:  oldPass,
						ChangedAt: now,
					})
				}

				if name != "" {
					entry.N = name
				}
				if user != "" {
					entry.U = user
				}
				if pass != "" {
					entry.P = pass
				}
				entry.Url = url
				entry.Nt = note
				entry.Tags = tags
				entry.LastModified = now

				v.E[i] = entry
				found = true
				break
			}
		}

		if !found {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "entry not found"})
			return
		}

		tempD, _ := json.Marshal(v)
		os.WriteFile(tempP, tempD, 0600)

		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "msg": "updated"})

	case "delete":
		if v == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "not unlocked"})
			return
		}

		id, _ := req["id"].(string)
		newE := []Entry{}
		for _, e := range v.E {
			if e.ID != id {
				newE = append(newE, e)
			}
		}
		v.E = newE

		tempD, _ := json.Marshal(v)
		os.WriteFile(tempP, tempD, 0600)

		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "msg": "deleted"})

	case "gen":
		l := 16
		if len, ok := req["len"].(float64); ok {
			l = int(len)
		}
		if l < 4 {
			l = 4
		}
		if l > 64 {
			l = 64
		}

		chars := "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*"
		pwd := make([]byte, l)
		for i := range pwd {
			pwd[i] = chars[time.Now().UnixNano()%int64(len(chars))]
			time.Sleep(time.Nanosecond)
		}
		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "data": string(pwd)})

	case "save":
		if v == nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "not unlocked"})
			return
		}

		tempD, err := json.Marshal(v)
		if err != nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "failed to serialize vault"})
			return
		}

		if err := os.WriteFile(tempP, tempD, 0600); err != nil {
			json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "failed to write temp file"})
			return
		}

		cmd := exec.Command(passlockBin, "sync", ms_pwd)
		output, err := cmd.CombinedOutput()

		if err != nil {
			json.NewEncoder(w).Encode(map[string]interface{}{
				"ok":  false,
				"msg": fmt.Sprintf("failed to save: %s", string(output)),
			})
			return
		}

		json.NewEncoder(w).Encode(map[string]interface{}{"ok": true, "msg": "saved to vault"})

	default:
		json.NewEncoder(w).Encode(map[string]interface{}{"ok": false, "msg": "unknown action"})
	}
}

func banner() {
	fmt.Println("╔═══════════════════════════════════════╗")
	fmt.Println("║       PASSLOCK WEB SERVER             ║")
	fmt.Println("║       http://localhost:8080           ║")
	fmt.Println("╚═══════════════════════════════════════╝")
}

func main() {
	banner()

	passlockBin := findPasslockBinary()
	fmt.Printf("→ Using PassLock binary: %s\n", passlockBin)

	http.Handle("/", http.FileServer(http.Dir("./web")))
	http.HandleFunc("/api", handle)
	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		json.NewEncoder(w).Encode(map[string]bool{"ok": true})
	})
	log.Println("→ Server started on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
