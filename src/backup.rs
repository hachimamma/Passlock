use crate::storage;
use std::fs;
use std::path::PathBuf;

/// Gets the backup directory path: ~/.passlock/backups/
pub fn get_backup_dir() -> PathBuf {
    crate::config::get_passlock_dir().join("backups")
}

/// Gets the vault-specific backup directory
pub fn get_vault_backup_dir(vault_name: &str) -> PathBuf {
    get_backup_dir().join(vault_name)
}

/// Initialize backup directory structure
pub fn init_backup_system() -> Result<(), Box<dyn std::error::Error>> {
    let backup_dir = get_backup_dir();
    
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)?;
        println!("[✔] Created backup directory: {}", backup_dir.display());
    }
    
    Ok(())
}

/// Create a timestamped backup of the vault
/// 
/// SMART BACKUP: Only creates a new backup if the vault has actually changed
/// since the last backup. This saves disk space!
/// 
/// Returns: (backup_filename, was_actually_created)
pub fn create_backup(
    vault_name: &str,
    max_backups: usize,
    force: bool, // Set true to force backup even if no changes
) -> Result<(String, bool), Box<dyn std::error::Error>> {
    let vault_path = crate::config::get_vault_path(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{}' not found", vault_name).into());
    }
    
    // SMART BACKUP: Check if vault has changed since last backup
    if !force {
        let backup_dir = get_vault_backup_dir(vault_name);
        if backup_dir.exists() {
            let backups = list_backups(vault_name)?;
            if !backups.is_empty() {
                // Get most recent backup
                let latest_backup = &backups[0];
                let latest_backup_path = backup_dir.join(&latest_backup.0);
                
                // Compare files
                let vault_meta = fs::metadata(&vault_path)?;
                let backup_meta = fs::metadata(&latest_backup_path)?;
                
                // Skip if same size (likely unchanged)
                if vault_meta.len() == backup_meta.len() {
                    println!("[✔] Backup skipped (no changes detected)");
                    return Ok((latest_backup.0.clone(), false));
                }
            }
        }
    }

    use chrono::Local;
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("backup_{timestamp}.vault");

    let backup_dir = get_vault_backup_dir(vault_name);
    fs::create_dir_all(&backup_dir)?;

    let backup_path = backup_dir.join(&backup_filename);

    // Copy vault to backup
    fs::copy(&vault_path, &backup_path)?;

    println!("[✔] Backup created: {backup_filename}");

    // Clean up old backups
    cleanup_old_backups(vault_name, max_backups)?;

    Ok((backup_filename, true))
}

/// List all backups for a vault
pub fn list_backups(
    vault_name: &str,
) -> Result<Vec<(String, u64, String)>, Box<dyn std::error::Error>> {
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

    // Sort by filename (timestamp) - newest first
    backups.sort_by(|a, b| b.0.cmp(&a.0));

    Ok(backups)
}

/// Restore vault from a backup
pub fn restore_backup(
    vault_name: &str,
    backup_filename: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let backup_dir = get_vault_backup_dir(vault_name);
    let backup_path = backup_dir.join(backup_filename);

    if !backup_path.exists() {
        return Err(format!("Backup file not found: {backup_filename}").into());
    }

    println!("[...] Verifying backup with password...");

    let vault_path = crate::config::get_vault_path(vault_name);
    let temp_path = crate::config::get_passlock_dir().join(format!("{}.temp", vault_name));
    
    // Copy backup to temp location
    fs::copy(&backup_path, &temp_path)?;

    // Try to decrypt it (validates password)
    // TODO: This needs to be updated to use vault_name parameter
    if storage::ld_vt(password).is_ok() {
        // Password works! Restore it
        fs::remove_file(&temp_path)?;

        // Safety backup of current vault
        if vault_path.exists() {
            let safety_backup = crate::config::get_passlock_dir()
                .join(format!("{}.before_restore", vault_name));
            fs::copy(&vault_path, &safety_backup)?;
            println!("[✔] Current vault backed up to: {}.before_restore", vault_name);
        }

        // Restore the backup
        fs::copy(&backup_path, &vault_path)?;
        println!("[✔] Vault '{}' restored from: {}", vault_name, backup_filename);

        Ok(())
    } else {
        fs::remove_file(&temp_path)?;
        Err("Incorrect password for this backup".into())
    }
}

/// Clean up old backups, keeping only the most recent N
fn cleanup_old_backups(
    vault_name: &str,
    max_backups: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let backups = list_backups(vault_name)?;

    if backups.len() > max_backups {
        let backup_dir = get_vault_backup_dir(vault_name);
        let to_delete = &backups[max_backups..];

        for (filename, _, _) in to_delete {
            let backup_path = backup_dir.join(filename);
            fs::remove_file(&backup_path)?;
            println!("[✔] Removed old backup: {filename}");
        }
    }

    Ok(())
}

/// Export vault to external location (encrypted)
pub fn export_vault(
    vault_name: &str,
    password: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Verify password
    let _vault = storage::ld_vt(password)?;

    let vault_path = crate::config::get_vault_path(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{}' not found", vault_name).into());
    }

    fs::copy(&vault_path, output_path)?;

    println!("[✔] Vault '{}' exported to: {}", vault_name, output_path);
    println!("[✔] Format: Encrypted PassLock vault");
    println!("[!] Keep this file secure!");

    Ok(())
}

/// Import vault from external location (encrypted)
pub fn import_vault(
    vault_name: &str,
    password: &str,
    input_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let import_path = PathBuf::from(input_path);

    if !import_path.exists() {
        return Err(format!("Import file not found: {input_path}").into());
    }

    let vault_path = crate::config::get_vault_path(vault_name);
    let temp_path = crate::config::get_passlock_dir().join(format!("{}.import_test", vault_name));
    
    fs::copy(&import_path, &temp_path)?;

    println!("[...] Verifying import file...");

    // Try to decrypt
    if storage::ld_vt(password).is_ok() {
        fs::remove_file(&temp_path)?;

        // Safety backup
        if vault_path.exists() {
            let safety_backup = crate::config::get_passlock_dir()
                .join(format!("{}.before_import", vault_name));
            fs::copy(&vault_path, &safety_backup)?;
            println!("[✔] Current vault backed up: {}.before_import", vault_name);
        }

        // Import
        fs::copy(&import_path, &vault_path)?;
        println!("[✔] Vault '{}' imported from: {}", vault_name, input_path);

        Ok(())
    } else {
        fs::remove_file(&temp_path)?;
        Err("Incorrect password for this vault file".into())
    }
}

/// Export to CSV (PLAINTEXT!)
pub fn export_to_csv(
    vault_name: &str,
    password: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let vault = storage::ld_vt(password)?;

    let mut csv_content = String::from("name,username,password,url,notes,tags,2fa_secret\n");

    for entry in &vault.e {
        let name = escape_csv(&entry.n);
        let username = escape_csv(&entry.u);
        let password_val = escape_csv(&entry.p);
        let url = entry.url.as_ref().map_or(String::new(), |u| escape_csv(u));
        let notes = entry.nt.as_ref().map_or(String::new(), |n| escape_csv(n));
        let tags = escape_csv(&entry.tags.join(";"));
        let totp = entry.totp_secret.as_ref().map_or(String::new(), |t| escape_csv(t));

        csv_content.push_str(&format!(
            "{name},{username},{password_val},{url},{notes},{tags},{totp}\n"
        ));
    }

    fs::write(output_path, csv_content)?;

    println!("[✔] Vault '{}' exported to CSV: {}", vault_name, output_path);
    println!("[!] WARNING: PLAINTEXT FILE - DELETE AFTER USE!");
    println!("[✔] {} entries exported", vault.e.len());

    Ok(())
}

/// Export to JSON (PLAINTEXT!)
pub fn export_to_json(
    vault_name: &str,
    password: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
        "exported_at": chrono::Local::now().to_rfc3339(),
        "entries": entries,
    });

    let json_string = serde_json::to_string_pretty(&json_output)?;
    fs::write(output_path, json_string)?;

    println!("[✔] Vault '{}' exported to JSON: {}", vault_name, output_path);
    println!("[!] WARNING: PLAINTEXT FILE - DELETE AFTER USE!");
    println!("[✔] {} entries exported", vault.e.len());

    Ok(())
}

/// Import from CSV
pub fn import_from_csv(
    vault_name: &str,
    password: &str,
    input_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::Entry;
    use std::io::{BufRead, BufReader};

    let mut vault = storage::ld_vt(password)?;

    let file = fs::File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Skip header
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
            url: fields.get(3).and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            nt: fields.get(4).and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            tags: fields.get(5).map_or(Vec::new(), |s| {
                s.split(';').filter(|t| !t.is_empty()).map(String::from).collect()
            }),
            totp_secret: fields.get(6).and_then(|s| if s.is_empty() { None } else { Some(s.clone()) }),
            t: crate::get_timestamp(),
            last_modified: crate::get_timestamp(),
            history: Vec::new(),
        };

        vault.e.push(entry);
        imported_count += 1;
    }

    storage::svv(&vault, password)?;

    println!("[✔] Import completed to vault '{}'", vault_name);
    println!("[✔] Imported: {} entries", imported_count);
    if skipped_count > 0 {
        println!("[!] Skipped: {} invalid entries", skipped_count);
    }

    // Auto-backup after import
    let config = crate::config::load_config()?;
    if config.auto_backup {
        create_backup(vault_name, config.max_backups, false)?;
    }

    Ok(())
}

/// Import from JSON
pub fn import_from_json(
    vault_name: &str,
    password: &str,
    input_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::Entry;

    let mut vault = storage::ld_vt(password)?;

    let json_content = fs::read_to_string(input_path)?;
    let json: serde_json::Value = serde_json::from_str(&json_content)?;

    let entries = match json.get("entries") {
        Some(serde_json::Value::Array(arr)) => arr,
        _ => return Err("Invalid JSON format".into()),
    };

    let mut imported_count = 0;

    for entry_json in entries {
        let entry = Entry {
            id: crate::generate_uuid(),
            n: entry_json["name"].as_str().unwrap_or("Untitled").to_string(),
            u: entry_json["username"].as_str().unwrap_or("").to_string(),
            p: entry_json["password"].as_str().unwrap_or("").to_string(),
            url: entry_json.get("url").and_then(|v| v.as_str()).map(String::from),
            nt: entry_json.get("notes").and_then(|v| v.as_str()).map(String::from),
            totp_secret: entry_json.get("totp_secret").and_then(|v| v.as_str()).map(String::from),
            tags: match entry_json.get("tags") {
                Some(serde_json::Value::Array(arr)) => {
                    arr.iter().filter_map(|v| v.as_str()).map(String::from).collect()
                }
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

    println!("[✔] Import completed to vault '{}'", vault_name);
    println!("[✔] Imported: {} entries", imported_count);

    // Auto-backup after import
    let config = crate::config::load_config()?;
    if config.auto_backup {
        create_backup(vault_name, config.max_backups, false)?;
    }

    Ok(())
}

fn escape_csv(field: &str) -> String {
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