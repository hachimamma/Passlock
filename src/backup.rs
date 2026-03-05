use crate::models::Entry;
use crate::storage;
use chrono::{DateTime, Local};
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

type BackupInfo = Vec<(String, u64, String)>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Get the backup directory path
pub fn gback_dir() -> PathBuf {
    crate::config::get_passlock_dir().join("backups")
}

/// Get the vault specific backup directory
pub fn gvback_dir(vault_name: &str) -> PathBuf {
    gback_dir().join(vault_name)
}

/// Init backup directory structure
pub fn init_bsys() -> Result<()> {
    let backup_dir = gback_dir();

    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)?;
        println!("[✔] Created backup directory: {}", backup_dir.display());
    }

    Ok(())
}

/// Create a timestamped backup of the vault
///
/// Only creates a new backup if the vault has actually changed
/// since the last backup
///
/// Returns: (`back_fn`, `was_actually_created`)
pub fn create_backup(vault_name: &str, max_backups: usize, force: bool) -> Result<(String, bool)> {
    let vault_path = crate::config::get_vault_path(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{vault_name}' not found").into());
    }

    if !force {
        let backup_dir = gvback_dir(vault_name);
        if backup_dir.exists() {
            let backups = ls_backs(vault_name)?;
            if !backups.is_empty() {
                let last_back = &backups[0];
                let lb_path = backup_dir.join(&last_back.0);

                let vault_meta = fs::metadata(&vault_path)?;
                let backup_meta = fs::metadata(&lb_path)?;

                if vault_meta.len() == backup_meta.len() {
                    println!("[✔] Backup skipped (no changes detected)");
                    return Ok((last_back.0.clone(), false));
                }
            }
        }
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let back_fn = format!("backup_{timestamp}.vault");

    let backup_dir = gvback_dir(vault_name);
    fs::create_dir_all(&backup_dir)?;

    let backup_path = backup_dir.join(&back_fn);

    fs::copy(&vault_path, &backup_path)?;

    println!("[✔] Backup created: {back_fn}");

    clean_obs(vault_name, max_backups)?;

    Ok((back_fn, true))
}

/// List all backups for a vault
pub fn ls_backs(vault_name: &str) -> Result<BackupInfo> {
    let backup_dir = gvback_dir(vault_name);

    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups = Vec::new();

    for entry in fs::read_dir(backup_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("vault") {
            let metadata = fs::metadata(&path)?;
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let size = metadata.len();

            let created = metadata.created()?;
            let datetime: DateTime<Local> = created.into();
            let created_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

            backups.push((filename, size, created_str));
        }
    }

    backups.sort_by(|a, b| b.0.cmp(&a.0));

    Ok(backups)
}

/// Restore vault from a back
pub fn restore_backup(vault_name: &str, back_fn: &str, password: &str) -> Result<()> {
    let backup_dir = gvback_dir(vault_name);
    let backup_path = backup_dir.join(back_fn);

    if !backup_path.exists() {
        return Err(format!("Backup file not found: {back_fn}").into());
    }

    println!("[...] Verifying backup with password...");

    let vault_path = crate::config::get_vault_path(vault_name);
    let temp_path = crate::config::get_passlock_dir().join(format!("{vault_name}.temp"));

    fs::copy(&backup_path, &temp_path)?;

    if storage::ld_vt(password).is_ok() {
        fs::remove_file(&temp_path)?;

        if vault_path.exists() {
            let safety_backup =
                crate::config::get_passlock_dir().join(format!("{vault_name}.before_restore"));
            fs::copy(&vault_path, &safety_backup)?;
            println!("[✔] Current vault backed up to: {vault_name}.before_restore");
        }

        fs::copy(&backup_path, &vault_path)?;
        println!("[✔] Vault '{vault_name}' restored from: {back_fn}");

        Ok(())
    } else {
        fs::remove_file(&temp_path)?;
        Err("Incorrect password for this backup".into())
    }
}

/// Clean up old backups, keeping recent N backs
fn clean_obs(vault_name: &str, max_backups: usize) -> Result<()> {
    let backups = ls_backs(vault_name)?;

    if backups.len() > max_backups {
        let backup_dir = gvback_dir(vault_name);
        let to_delete = &backups[max_backups..];

        for (filename, _, _) in to_delete {
            let backup_path = backup_dir.join(filename);
            fs::remove_file(&backup_path)?;
            println!("[✔] Removed old backup: {filename}");
        }
    }

    Ok(())
}

/// Export vault to external location
pub fn export_vault(vault_name: &str, password: &str, output_path: &str) -> Result<()> {
    let _vault = storage::ld_vt(password)?;

    let vault_path = crate::config::get_vault_path(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{vault_name}' not found").into());
    }

    fs::copy(&vault_path, output_path)?;

    println!("[✔] Vault '{vault_name}' exported to: {output_path}");
    println!("[✔] Format: Encrypted PassLock vault");
    println!("[!] Keep this file secure.");

    Ok(())
}

/// Import vault from external location
pub fn _import_vault(vault_name: &str, password: &str, input_path: &str) -> Result<()> {
    let import_path = PathBuf::from(input_path);

    if !import_path.exists() {
        return Err(format!("Import file not found: {input_path}").into());
    }

    let vault_path = crate::config::get_vault_path(vault_name);
    let temp_path = crate::config::get_passlock_dir().join(format!("{vault_name}.import_test"));

    fs::copy(&import_path, &temp_path)?;

    println!("[...] Verifying import file...");

    if storage::ld_vt(password).is_ok() {
        fs::remove_file(&temp_path)?;

        if vault_path.exists() {
            let safety_backup =
                crate::config::get_passlock_dir().join(format!("{vault_name}.before_import"));
            fs::copy(&vault_path, &safety_backup)?;
            println!("[✔] Current vault backed up: {vault_name}.before_import");
        }

        fs::copy(&import_path, &vault_path)?;
        println!("[✔] Vault '{vault_name}' imported from: {input_path}");

        Ok(())
    } else {
        fs::remove_file(&temp_path)?;
        Err("Incorrect password for this vault file".into())
    }
}

/// Export to CSV
pub fn export_csv(vault_name: &str, password: &str, output_path: &str) -> Result<()> {
    let vault = storage::ld_vt(password)?;

    let mut csv_content = String::from("name,username,password,url,notes,tags,2fa_secret\n");

    for entry in &vault.e {
        let name = esc_csv(&entry.n);
        let username = esc_csv(&entry.u);
        let password_val = esc_csv(&entry.p);
        let url = entry.url.as_ref().map_or(String::new(), |u| esc_csv(u));
        let notes = entry.nt.as_ref().map_or(String::new(), |n| esc_csv(n));
        let tags = esc_csv(&entry.tags.join(";"));
        let totp = entry
            .totp_secret
            .as_ref()
            .map_or(String::new(), |t| esc_csv(t));

        writeln!(
            csv_content,
            "{name},{username},{password_val},{url},{notes},{tags},{totp}"
        )?;
    }

    fs::write(output_path, csv_content)?;

    println!("[✔] Vault '{vault_name}' exported to CSV: {output_path}");
    println!("[!] WARNING: This is Plaintext, delete after use.");
    println!("[✔] {} entries exported", vault.e.len());

    Ok(())
}

/// Export to JSON
pub fn export_json(vault_name: &str, password: &str, output_path: &str) -> Result<()> {
    let vault = storage::ld_vt(password)?;

    let mut entries = Vec::new();
    for entry in &vault.e {
        let mut json_entry = serde_json::json!({
            "name": entry.n,
            "username": entry.u,
            "password": entry.p,
            "tags": entry.tags,
        });

        if let Some(ref url) = entry.url {
            json_entry["url"] = serde_json::json!(url);
        }
        if let Some(ref notes) = entry.nt {
            json_entry["notes"] = serde_json::json!(notes);
        }
        if let Some(ref totp) = entry.totp_secret {
            json_entry["totp_secret"] = serde_json::json!(totp);
        }

        entries.push(json_entry);
    }

    let json_output = serde_json::json!({
        "passlock_version": env!("CARGO_PKG_VERSION"),
        "vault_name": vault_name,
        "exported_at": Local::now().to_rfc3339(),
        "entries": entries,
    });

    let json_string = serde_json::to_string_pretty(&json_output)?;
    fs::write(output_path, json_string)?;

    println!("[✔] Vault '{vault_name}' exported to JSON: {output_path}");
    println!("[!] WARNING: This is Plaintext, delete after use.");
    println!("[✔] {} entries exported", vault.e.len());

    Ok(())
}

/// Import from CSV
pub fn import_csv(vault_name: &str, password: &str, input_path: &str) -> Result<()> {
    use std::io::{BufRead, BufReader};

    let mut vault = storage::ld_vt(password)?;

    let file = fs::File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    if let Some(Ok(header)) = lines.next() {
        if !header.contains("name") || !header.contains("password") {
            return Err("Invalid CSV format".into());
        }
    } else {
        return Err("Empty CSV file".into());
    }

    let mut imported_count = 0;
    let mut skipped_count = 0;

    for line_result in lines {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }

        let fields = parse_csv_line(&line);

        if fields.len() < 3 {
            skipped_count += 1;
            continue;
        }

        let entry = Entry {
            id: crate::generate_uuid(),
            n: fields[0].clone(),
            u: fields.get(1).cloned().unwrap_or_default(),
            p: fields.get(2).cloned().unwrap_or_default(),
            url: fields
                .get(3)
                .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            nt: fields
                .get(4)
                .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            tags: fields.get(5).map_or(Vec::new(), |s| {
                s.split(';')
                    .filter(|t| !t.is_empty())
                    .map(String::from)
                    .collect()
            }),
            totp_secret: fields
                .get(6)
                .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            t: crate::get_timestamp(),
            last_modified: crate::get_timestamp(),
            history: Vec::new(),
        };

        vault.e.push(entry);
        imported_count += 1;
    }

    storage::svv(&vault, password)?;

    println!("[✔] Import completed to vault '{vault_name}'");
    println!("[✔] Imported: {imported_count} entries");
    if skipped_count > 0 {
        println!("[!] Skipped: {skipped_count} invalid entries");
    }

    let config = crate::config::load_config()?;
    if config.auto_backup {
        create_backup(vault_name, config.max_backups, false)?;
    }

    Ok(())
}

/// Import from JSON
pub fn import_json(vault_name: &str, password: &str, input_path: &str) -> Result<()> {
    let mut vault = storage::ld_vt(password)?;

    let json_content = fs::read_to_string(input_path)?;
    let json: serde_json::Value = serde_json::from_str(&json_content)?;

    let Some(serde_json::Value::Array(entries)) = json.get("entries") else {
        return Err("Invalid JSON format".into());
    };

    let mut imported_count = 0;

    for entry_json in entries {
        let entry = Entry {
            id: crate::generate_uuid(),
            n: entry_json["name"]
                .as_str()
                .unwrap_or("Untitled")
                .to_string(),
            u: entry_json["username"].as_str().unwrap_or("").to_string(),
            p: entry_json["password"].as_str().unwrap_or("").to_string(),
            url: entry_json
                .get("url")
                .and_then(|v| v.as_str())
                .map(String::from),
            nt: entry_json
                .get("notes")
                .and_then(|v| v.as_str())
                .map(String::from),
            totp_secret: entry_json
                .get("totp_secret")
                .and_then(|v| v.as_str())
                .map(String::from),
            tags: match entry_json.get("tags") {
                Some(serde_json::Value::Array(arr)) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect(),
                _ => Vec::new(),
            },
            t: crate::get_timestamp(),
            last_modified: crate::get_timestamp(),
            history: Vec::new(),
        };

        vault.e.push(entry);
        imported_count += 1;
    }

    storage::svv(&vault, password)?;

    println!("[✔] Import completed to vault '{vault_name}'");
    println!("[✔] Imported: {imported_count} entries");

    let config = crate::config::load_config()?;
    if config.auto_backup {
        create_backup(vault_name, config.max_backups, false)?;
    }

    Ok(())
}

fn esc_csv(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    current_field.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                fields.push(current_field.trim().to_string());
                current_field.clear();
            }
            _ => {
                current_field.push(ch);
            }
        }
    }

    fields.push(current_field.trim().to_string());
    fields
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ImportPreviewEntry {
    pub name: String,
    pub username: String,
    pub url: Option<String>,
    pub has_password: bool,
    pub has_totp: bool,
    pub tags: Vec<String>,
    pub is_duplicate: bool,
    pub notes_preview: Option<String>,
}

#[derive(Debug)]
pub struct ImportPreview {
    pub total_entries: usize,
    pub valid_entries: usize,
    pub empty_entries: usize,
    pub duplicates: usize,
    pub entries: Vec<ImportPreviewEntry>,
    pub errors: Vec<String>,
}

/// Preview CSV
pub fn preview_csv_import(
    _vault_name: &str,
    password: &str,
    input_path: &str,
) -> Result<ImportPreview> {
    use std::io::{BufRead, BufReader};

    let vault = storage::ld_vt(password)?;

    let file = fs::File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    if let Some(Ok(header)) = lines.next() {
        if !header.contains("name") || !header.contains("password") {
            return Err("Invalid CSV format: missing required columns (name, password)".into());
        }
    } else {
        return Err("Empty CSV file".into());
    }

    let mut preview = ImportPreview {
        total_entries: 0,
        valid_entries: 0,
        empty_entries: 0,
        duplicates: 0,
        entries: Vec::new(),
        errors: Vec::new(),
    };

    for (line_num, line_result) in lines.enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                preview
                    .errors
                    .push(format!("Line {}: Read error - {}", line_num + 2, e));
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        preview.total_entries += 1;

        let fields = parse_csv_line(&line);

        if fields.len() < 3 {
            preview.errors.push(format!(
                "Line {}: Insufficient fields (need at least name, username, password)",
                line_num + 2
            ));
            preview.empty_entries += 1;
            continue;
        }

        let name = fields[0].clone();
        let username = fields.get(1).cloned().unwrap_or_default();
        let password_val = fields.get(2).cloned().unwrap_or_default();

        if name.is_empty() || password_val.is_empty() {
            preview
                .errors
                .push(format!("Line {}: Empty name or password", line_num + 2));
            preview.empty_entries += 1;
            continue;
        }

        let is_duplicate = vault.e.iter().any(|e| {
            e.n.to_lowercase() == name.to_lowercase()
                && e.u.to_lowercase() == username.to_lowercase()
        });

        if is_duplicate {
            preview.duplicates += 1;
        }

        let url = fields
            .get(3)
            .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });
        let notes = fields
            .get(4)
            .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });
        let tags: Vec<String> = fields.get(5).map_or(Vec::new(), |s| {
            s.split(';')
                .filter(|t| !t.is_empty())
                .map(String::from)
                .collect()
        });
        let totp_secret = fields
            .get(6)
            .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });

        let notes_preview = notes.as_ref().map(|n| {
            if n.len() > 50 {
                format!("{}...", &n[..50])
            } else {
                n.clone()
            }
        });

        preview.entries.push(ImportPreviewEntry {
            name,
            username,
            url,
            has_password: !password_val.is_empty(),
            has_totp: totp_secret.is_some(),
            tags,
            is_duplicate,
            notes_preview,
        });

        preview.valid_entries += 1;
    }

    Ok(preview)
}

/// Preview JSON
pub fn preview_json_import(
    _vault_name: &str,
    password: &str,
    input_path: &str,
) -> Result<ImportPreview> {
    let vault = storage::ld_vt(password)?;

    let json_content = fs::read_to_string(input_path)?;
    let json: serde_json::Value = serde_json::from_str(&json_content)?;

    let Some(serde_json::Value::Array(entries)) = json.get("entries") else {
        return Err("Invalid JSON format: missing 'entries' array".into());
    };

    let mut preview = ImportPreview {
        total_entries: entries.len(),
        valid_entries: 0,
        empty_entries: 0,
        duplicates: 0,
        entries: Vec::new(),
        errors: Vec::new(),
    };

    for (idx, entry_json) in entries.iter().enumerate() {
        let name = entry_json["name"]
            .as_str()
            .unwrap_or("Untitled")
            .to_string();
        let username = entry_json["username"].as_str().unwrap_or("").to_string();
        let password_val = entry_json["password"].as_str().unwrap_or("").to_string();

        if name.is_empty() || password_val.is_empty() {
            preview
                .errors
                .push(format!("Entry {}: Empty name or password", idx + 1));
            preview.empty_entries += 1;
            continue;
        }

        let is_duplicate = vault.e.iter().any(|e| {
            e.n.to_lowercase() == name.to_lowercase()
                && e.u.to_lowercase() == username.to_lowercase()
        });

        if is_duplicate {
            preview.duplicates += 1;
        }

        let url = entry_json
            .get("url")
            .and_then(|v| v.as_str())
            .map(String::from);
        let notes = entry_json
            .get("notes")
            .and_then(|v| v.as_str())
            .map(String::from);
        let totp_secret = entry_json
            .get("totp_secret")
            .and_then(|v| v.as_str())
            .map(String::from);
        let tags: Vec<String> = match entry_json.get("tags") {
            Some(serde_json::Value::Array(arr)) => arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect(),
            _ => Vec::new(),
        };

        let notes_preview = notes.as_ref().map(|n| {
            if n.len() > 50 {
                format!("{}...", &n[..50])
            } else {
                n.clone()
            }
        });

        preview.entries.push(ImportPreviewEntry {
            name,
            username,
            url,
            has_password: !password_val.is_empty(),
            has_totp: totp_secret.is_some(),
            tags,
            is_duplicate,
            notes_preview,
        });

        preview.valid_entries += 1;
    }

    Ok(preview)
}

/// Import CSV with duplicate handling
pub fn import_csv_smart(
    vault_name: &str,
    password: &str,
    input_path: &str,
    skip_duplicates: bool,
    merge_duplicates: bool,
) -> Result<(usize, usize)> {
    use std::io::{BufRead, BufReader};

    let mut vault = storage::ld_vt(password)?;

    let file = fs::File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    if let Some(Ok(header)) = lines.next() {
        if !header.contains("name") || !header.contains("password") {
            return Err("Invalid CSV format".into());
        }
    } else {
        return Err("Empty CSV file".into());
    }

    let mut imported_count = 0;
    let mut skipped_count = 0;

    for line_result in lines {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }

        let fields = parse_csv_line(&line);

        if fields.len() < 3 {
            skipped_count += 1;
            continue;
        }

        let name = fields[0].clone();
        let username = fields.get(1).cloned().unwrap_or_default();
        let password_val = fields.get(2).cloned().unwrap_or_default();

        if name.is_empty() || password_val.is_empty() {
            skipped_count += 1;
            continue;
        }

        if let Some(existing_idx) = vault.e.iter().position(|e| {
            e.n.to_lowercase() == name.to_lowercase()
                && e.u.to_lowercase() == username.to_lowercase()
        }) {
            if skip_duplicates {
                skipped_count += 1;
                continue;
            } else if merge_duplicates {
                let entry = &mut vault.e[existing_idx];
                entry.p = password_val.clone();

                if let Some(url) = fields.get(3).filter(|s| !s.is_empty()) {
                    entry.url = Some(url.clone());
                }
                if let Some(notes) = fields.get(4).filter(|s| !s.is_empty()) {
                    entry.nt = Some(notes.clone());
                }
                if let Some(tags_str) = fields.get(5).filter(|s| !s.is_empty()) {
                    let new_tags: Vec<String> = tags_str
                        .split(';')
                        .filter(|t| !t.is_empty())
                        .map(String::from)
                        .collect();
                    entry.tags = new_tags;
                }
                if let Some(totp) = fields.get(6).filter(|s| !s.is_empty()) {
                    entry.totp_secret = Some(totp.clone());
                }

                entry.last_modified = crate::get_timestamp();
                imported_count += 1;
                continue;
            }
        }

        let entry = Entry {
            id: crate::generate_uuid(),
            n: name,
            u: username,
            p: password_val,
            url: fields
                .get(3)
                .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            nt: fields
                .get(4)
                .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            tags: fields.get(5).map_or(Vec::new(), |s| {
                s.split(';')
                    .filter(|t| !t.is_empty())
                    .map(String::from)
                    .collect()
            }),
            totp_secret: fields
                .get(6)
                .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            t: crate::get_timestamp(),
            last_modified: crate::get_timestamp(),
            history: Vec::new(),
        };

        vault.e.push(entry);
        imported_count += 1;
    }

    storage::svv(&vault, password)?;

    let config = crate::config::load_config()?;
    if config.auto_backup {
        create_backup(vault_name, config.max_backups, false)?;
    }

    Ok((imported_count, skipped_count))
}

/// Export CSV with filters
pub fn export_csv_filtered(
    _vault_name: &str,
    password: &str,
    output_path: &str,
    filter_tag: Option<&str>,
    filter_search: Option<&str>,
) -> Result<()> {
    let vault = storage::ld_vt(password)?;

    let filtered_entries: Vec<&Entry> = vault
        .e
        .iter()
        .filter(|e| {
            if let Some(tag) = filter_tag {
                if !e.tags.contains(&tag.to_string()) {
                    return false;
                }
            }

            if let Some(search) = filter_search {
                let query = search.to_lowercase();
                if !e.n.to_lowercase().contains(&query)
                    && !e.u.to_lowercase().contains(&query)
                    && !e
                        .url
                        .as_ref()
                        .map_or(false, |u| u.to_lowercase().contains(&query))
                {
                    return false;
                }
            }

            true
        })
        .collect();

    if filtered_entries.is_empty() {
        return Err("No entries match the filter criteria".into());
    }

    let mut csv_content = String::from("name,username,password,url,notes,tags,2fa_secret\n");

    for entry in &filtered_entries {
        let name = esc_csv(&entry.n);
        let username = esc_csv(&entry.u);
        let password_val = esc_csv(&entry.p);
        let url = entry.url.as_ref().map_or(String::new(), |u| esc_csv(u));
        let notes = entry.nt.as_ref().map_or(String::new(), |n| esc_csv(n));
        let tags = esc_csv(&entry.tags.join(";"));
        let totp = entry
            .totp_secret
            .as_ref()
            .map_or(String::new(), |t| esc_csv(t));

        writeln!(
            csv_content,
            "{name},{username},{password_val},{url},{notes},{tags},{totp}"
        )?;
    }

    fs::write(output_path, csv_content)?;

    println!(
        "[✔] Exported {} entries to CSV: {}",
        filtered_entries.len(),
        output_path
    );
    println!("[!] WARNING: This is Plaintext, delete after use.");

    Ok(())
}
