use crate::storage;
use std::fs;
use std::path::PathBuf;

/// Gets the backup directory path
pub fn gback_dir() -> PathBuf {
    crate::config::get_passlock_dir().join("backups")
}

/// Gets the vault specific backup directory
pub fn gvback_dir(vault_name: &str) -> PathBuf {
    gback_dir().join(vault_name)
}

/// Initialize backup directory structure
pub fn init_bsys() -> Result<(), Box<dyn std::error::Error>> {
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
/// Returns: (back_fn, was_actually_created)
pub fn create_backup(
    vault_name: &str,
    max_backups: usize,
    force: bool,
) -> Result<(String, bool), Box<dyn std::error::Error>> {
    let vault_path = crate::config::get_vault_path(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{}' not found", vault_name).into());
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

    use chrono::Local;
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
pub fn ls_backs(
    vault_name: &str,
) -> Result<Vec<(String, u64, String)>, Box<dyn std::error::Error>> {
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

/// Restore vault from a back
pub fn restore_backup(
    vault_name: &str,
    back_fn: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let backup_dir = gvback_dir(vault_name);
    let backup_path = backup_dir.join(back_fn);

    if !backup_path.exists() {
        return Err(format!("Backup file not found: {back_fn}").into());
    }

    println!("[...] Verifying backup with password...");

    let vault_path = crate::config::get_vault_path(vault_name);
    let temp_path = crate::config::get_passlock_dir().join(format!("{}.temp", vault_name));
    
    fs::copy(&backup_path, &temp_path)?;

    if storage::ld_vt(password).is_ok() {
        fs::remove_file(&temp_path)?;

        if vault_path.exists() {
            let safety_backup = crate::config::get_passlock_dir()
                .join(format!("{}.before_restore", vault_name));
            fs::copy(&vault_path, &safety_backup)?;
            println!("[✔] Current vault backed up to: {}.before_restore", vault_name);
        }

        fs::copy(&backup_path, &vault_path)?;
        println!("[✔] Vault '{}' restored from: {}", vault_name, back_fn);

        Ok(())
    } else {
        fs::remove_file(&temp_path)?;
        Err("Incorrect password for this backup".into())
    }
}

/// Clean up old backups, keeping recent N backs
fn clean_obs(
    vault_name: &str,
    max_backups: usize,
) -> Result<(), Box<dyn std::error::Error>> {
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
pub fn export_vault(
    vault_name: &str,
    password: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let _vault = storage::ld_vt(password)?;

    let vault_path = crate::config::get_vault_path(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{}' not found", vault_name).into());
    }

    fs::copy(&vault_path, output_path)?;

    println!("[✔] Vault '{}' exported to: {}", vault_name, output_path);
    println!("[✔] Format: Encrypted PassLock vault");
    println!("[!] Keep this file secure.");

    Ok(())
}

/// Import vault from external location
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

    if storage::ld_vt(password).is_ok() {
        fs::remove_file(&temp_path)?;

        if vault_path.exists() {
            let safety_backup = crate::config::get_passlock_dir()
                .join(format!("{}.before_import", vault_name));
            fs::copy(&vault_path, &safety_backup)?;
            println!("[✔] Current vault backed up: {}.before_import", vault_name);
        }

        fs::copy(&import_path, &vault_path)?;
        println!("[✔] Vault '{}' imported from: {}", vault_name, input_path);

        Ok(())
    } else {
        fs::remove_file(&temp_path)?;
        Err("Incorrect password for this vault file".into())
    }
}

/// Export to CSV
pub fn export_csv(
    vault_name: &str,
    password: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let vault = storage::ld_vt(password)?;

    let mut csv_content = String::from("name,username,password,url,notes,tags,2fa_secret\n");

    for entry in &vault.e {
        let name = esc_csv(&entry.n);
        let username = esc_csv(&entry.u);
        let password_val = esc_csv(&entry.p);
        let url = entry.url.as_ref().map_or(String::new(), |u| esc_csv(u));
        let notes = entry.nt.as_ref().map_or(String::new(), |n| esc_csv(n));
        let tags = esc_csv(&entry.tags.join(";"));
        let totp = entry.totp_secret.as_ref().map_or(String::new(), |t| esc_csv(t));

        csv_content.push_str(&format!(
            "{name},{username},{password_val},{url},{totp},{tags},{notes}\n"
        ));
    }

    fs::write(output_path, csv_content)?;

    println!("[✔] Vault '{}' exported to CSV: {}", vault_name, output_path);
    println!("[!] WARNING: This is Plaintext, delete after use.");
    println!("[✔] {} entries exported", vault.e.len());

    Ok(())
}

/// Export to JSON
pub fn export_json(
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
    println!("[!] WARNING: This is Plaintext, delete after use.");
    println!("[✔] {} entries exported", vault.e.len());

    Ok(())
}

/// Import from CSV
pub fn import_csv(
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

    let config = crate::config::load_config()?;
    if config.auto_backup {
        create_backup(vault_name, config.max_backups, false)?;
    }

    Ok(())
}

/// Import from JSON
pub fn import_json(
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