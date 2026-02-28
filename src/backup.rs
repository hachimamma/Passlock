use crate::storage;
use std::fs;
use std::path::PathBuf;

/// Gets the backup directory path: ~/.passlock/backups/
/// This is where all automatic backups will be stored
pub fn get_backup_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".passlock").join("backups")
}

/// Gets the vault-specific backup directory
/// If you have multiple vaults later, each gets its own backup folder
pub fn get_vault_backup_dir(vault_name: &str) -> PathBuf {
    get_backup_dir().join(vault_name)
}

/// Initialize backup directory structure
/// Creates the folders if they don't exist
pub fn init_backup_system() -> Result<(), Box<dyn std::error::Error>> {
    let backup_dir = get_backup_dir();
    
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)?;
        println!("[✔] Created backup directory: {}", backup_dir.display());
    }
    
    let default_backup_dir = get_vault_backup_dir("default");
    if !default_backup_dir.exists() {
        fs::create_dir_all(&default_backup_dir)?;
        println!("[✔] Created vault backup directory: {}", default_backup_dir.display());
    }
    
    Ok(())
}

/// Create a timestamped backup of the vault
/// Format: backup_2026-02-28_18-30-45.vault
/// 
/// This function:
/// 1. Gets current timestamp
/// 2. Creates a filename with that timestamp
/// 3. Copies the vault file to the backup directory
/// 4. Manages old backups (keeps only last N backups)
pub fn create_backup(vault_name: &str, max_backups: usize) -> Result<String, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().expect("Could not find home directory");
    let vault_path = home.join(".passlock.vault");
    
    if !vault_path.exists() {
        return Err("No vault file found to backup".into());
    }
    
    use chrono::Local;
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("backup_{}.vault", timestamp);
    
    let backup_dir = get_vault_backup_dir(vault_name);
    fs::create_dir_all(&backup_dir)?;
    
    let backup_path = backup_dir.join(&backup_filename);
    
    fs::copy(&vault_path, &backup_path)?;
    
    println!("[✔] Backup created: {}", backup_filename);
    
    cleanup_old_backups(vault_name, max_backups)?;
    
    Ok(backup_filename)
}

/// List all backups for a vault
/// Returns a vector of (filename, file_size, created_time)
pub fn list_backups(vault_name: &str) -> Result<Vec<(String, u64, String)>, Box<dyn std::error::Error>> {
    let backup_dir = get_vault_backup_dir(vault_name);
    
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
            
            use chrono::{DateTime, Local};
            let created = metadata.created()?;
            let datetime: DateTime<Local> = created.into();
            let created_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            
            backups.push((filename, size, created_str));
        }
    }
    
    backups.sort_by(|a, b| b.0.cmp(&a.0));
    
    Ok(backups)
}

/// Restore vault from a backup
/// 
/// This function:
/// 1. Finds the backup file
/// 2. Verifies it can be unlocked with the provided password
/// 3. Copies it back to the main vault location
pub fn restore_backup(vault_name: &str, backup_filename: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let backup_dir = get_vault_backup_dir(vault_name);
    let backup_path = backup_dir.join(backup_filename);
    
    if !backup_path.exists() {
        return Err(format!("Backup file not found: {}", backup_filename).into());
    }
    
    println!("[...] Verifying backup with password...");
    
    let home = dirs::home_dir().expect("Could not find home directory");
    let test_path = home.join(".passlock.vault.test");
    fs::copy(&backup_path, &test_path)?;
    
    match storage::ld_vt(password) {
        Ok(_) => {
            fs::remove_file(&test_path)?;
            
            let vault_path = home.join(".passlock.vault");
            
            if vault_path.exists() {
                let safety_backup = home.join(".passlock.vault.before_restore");
                fs::copy(&vault_path, &safety_backup)?;
                println!("[✔] Current vault backed up to: .passlock.vault.before_restore");
            }
            
            fs::copy(&backup_path, &vault_path)?;
            println!("[✔] Vault restored from backup: {}", backup_filename);
            
            Ok(())
        }
        Err(_) => {
            fs::remove_file(&test_path)?;
            Err("Incorrect password for this backup".into())
        }
    }
}

/// Clean up old backups, keeping only the most recent N backups
fn cleanup_old_backups(vault_name: &str, max_backups: usize) -> Result<(), Box<dyn std::error::Error>> {
    let backups = list_backups(vault_name)?;
    
    if backups.len() > max_backups {
        let backup_dir = get_vault_backup_dir(vault_name);
        let to_delete = &backups[max_backups..];
        
        for (filename, _, _) in to_delete {
            let backup_path = backup_dir.join(filename);
            fs::remove_file(&backup_path)?;
            println!("[✔] Removed old backup: {}", filename);
        }
    }
    
    Ok(())
}

/// Export vault to a specific location (encrypted PassLock format)
/// Useful for manual backups or transferring between systems
pub fn export_vault(password: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _vault = storage::ld_vt(password)?;
    
    let home = dirs::home_dir().expect("Could not find home directory");
    let vault_path = home.join(".passlock.vault");
    
    if !vault_path.exists() {
        return Err("No vault file found to export".into());
    }
    
    fs::copy(&vault_path, output_path)?;
    
    println!("[✔] Vault exported to: {}", output_path);
    println!("[✔] Format: Encrypted PassLock vault");
    println!("[!] Keep this file secure - it contains all your passwords!");
    
    Ok(())
}

/// Export vault to CSV format (PLAINTEXT - for importing to other password managers)
/// WARNING: This creates an UNENCRYPTED file with all passwords visible!
pub fn export_to_csv(password: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vault = storage::ld_vt(password)?;
    
    let mut csv_content = String::from("name,username,password,url,notes,tags,2fa_secret\n");
    
    for entry in &vault.e {
        let name = escape_csv(&entry.n);
        let username = escape_csv(&entry.u);
        let password = escape_csv(&entry.p);
        let url = entry.url.as_ref().map_or(String::new(), |u| escape_csv(u));
        let notes = entry.nt.as_ref().map_or(String::new(), |n| escape_csv(n));
        let tags = escape_csv(&entry.tags.join(";"));
        let totp = entry.totp_secret.as_ref().map_or(String::new(), |t| escape_csv(t));
        
        csv_content.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            name, username, password, url, notes, tags, totp
        ));
    }
    
    fs::write(output_path, csv_content)?;
    
    println!("[✔] Vault exported to CSV: {}", output_path);
    println!("[!] WARNING: This file is UNENCRYPTED and contains plaintext passwords!");
    println!("[!] Use this for importing to other password managers, then DELETE it!");
    println!("[✔] Total entries exported: {}", vault.e.len());
    
    Ok(())
}

/// Export vault to JSON format (PLAINTEXT - for importing to other password managers)
/// WARNING: This creates an UNENCRYPTED file with all passwords visible!
pub fn export_to_json(password: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        "exported_at": chrono::Local::now().to_rfc3339(),
        "entries": entries,
    });
    
    let json_string = serde_json::to_string_pretty(&json_output)?;
    fs::write(output_path, json_string)?;
    
    println!("[✔] Vault exported to JSON: {}", output_path);
    println!("[!] WARNING: This file is UNENCRYPTED and contains plaintext passwords!");
    println!("[!] Use this for importing to other password managers, then DELETE it!");
    println!("[✔] Total entries exported: {}", vault.e.len());
    
    Ok(())
}

/// Helper function to escape CSV fields
fn escape_csv(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// Import from CSV file (from other password managers like LastPass, Bitwarden, etc.)
/// Expected format: name,username,password,url,notes,tags,2fa_secret
pub fn import_from_csv(password: &str, input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::Entry;
    use std::io::{BufRead, BufReader};
    
    let mut vault = storage::ld_vt(password)?;
    
    let file = fs::File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    if let Some(Ok(header)) = lines.next() {
        if !header.contains("name") || !header.contains("password") {
            return Err("Invalid CSV format - expected header with 'name' and 'password'".into());
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
            println!("[!] Skipping invalid line: {}", line);
            skipped_count += 1;
            continue;
        }
        
        let name = fields[0].clone();
        let username = fields.get(1).cloned().unwrap_or_default();
        let password_val = fields.get(2).cloned().unwrap_or_default();
        let url = fields.get(3).and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });
        let notes = fields.get(4).and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });
        let tags_str = fields.get(5).cloned().unwrap_or_default();
        let totp_secret = fields.get(6).and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });
        
        let tags: Vec<String> = tags_str
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        let entry = Entry {
            id: crate::generate_uuid(),
            n: name,
            u: username,
            p: password_val,
            url,
            nt: notes,
            tags,
            totp_secret,
            t: crate::get_timestamp(),
            last_modified: crate::get_timestamp(),
            history: Vec::new(),
        };
        
        vault.e.push(entry);
        imported_count += 1;
    }
    
    storage::svv(&vault, password)?;
    
    println!("[✔] Import completed!");
    println!("[✔] Imported: {} entries", imported_count);
    if skipped_count > 0 {
        println!("[!] Skipped: {} invalid entries", skipped_count);
    }
    println!("[✔] Total entries in vault: {}", vault.e.len());
    
    create_backup("default", 10)?;
    
    Ok(())
}

pub fn import_from_json(password: &str, input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::Entry;
    
    let mut vault = storage::ld_vt(password)?;
    
    let json_content = fs::read_to_string(input_path)?;
    let json: serde_json::Value = serde_json::from_str(&json_content)?;
    
    let entries = match json.get("entries") {
        Some(serde_json::Value::Array(arr)) => arr,
        _ => return Err("Invalid JSON format - expected 'entries' array".into()),
    };
    
    let mut imported_count = 0;
    
    for entry_json in entries {
        let name = entry_json["name"].as_str().unwrap_or("Untitled").to_string();
        let username = entry_json["username"].as_str().unwrap_or("").to_string();
        let password_val = entry_json["password"].as_str().unwrap_or("").to_string();
        let url = entry_json.get("url").and_then(|v| v.as_str()).map(String::from);
        let notes = entry_json.get("notes").and_then(|v| v.as_str()).map(String::from);
        let totp_secret = entry_json.get("totp_secret").and_then(|v| v.as_str()).map(String::from);
        
        let tags = match entry_json.get("tags") {
            Some(serde_json::Value::Array(arr)) => {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            }
            _ => Vec::new(),
        };
        
        let entry = Entry {
            id: crate::generate_uuid(),
            n: name,
            u: username,
            p: password_val,
            url,
            nt: notes,
            tags,
            totp_secret,
            t: crate::get_timestamp(),
            last_modified: crate::get_timestamp(),
            history: Vec::new(),
        };
        
        vault.e.push(entry);
        imported_count += 1;
    }
    
    storage::svv(&vault, password)?;
    
    println!("[✔] Import completed!");
    println!("[✔] Imported: {} entries", imported_count);
    println!("[✔] Total entries in vault: {}", vault.e.len());

    create_backup("default", 10)?;
    
    Ok(())
}

/// Simple CSV line parser that handles quoted fields
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

/// Import vault from a specific location
/// Replaces current vault with the imported one
pub fn import_vault(password: &str, input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let import_path = PathBuf::from(input_path);
    
    if !import_path.exists() {
        return Err(format!("Import file not found: {}", input_path).into());
    }
    
    let home = dirs::home_dir().expect("Could not find home directory");
    let temp_path = home.join(".passlock.vault.import_test");
    fs::copy(&import_path, &temp_path)?;
    
    println!("[...] Verifying import file with password...");
    
    match storage::ld_vt(password) {
        Ok(_) => {
            fs::remove_file(&temp_path)?;
            
            let vault_path = home.join(".passlock.vault");
            
            if vault_path.exists() {
                let safety_backup = home.join(".passlock.vault.before_import");
                fs::copy(&vault_path, &safety_backup)?;
                println!("[✔] Current vault backed up to: .passlock.vault.before_import");
            }
            
            fs::copy(&import_path, &vault_path)?;
            println!("[✔] Vault imported from: {}", input_path);
            
            Ok(())
        }
        Err(_) => {
            fs::remove_file(&temp_path)?;
            Err("Incorrect password for this vault file".into())
        }
    }
}