use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub active_vault: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    pub auto_backup: bool,
    pub max_backups: usize,
    pub clipboard_timeout: u64,
    pub refresh_rate: u64,
}

fn default_theme() -> String {
    String::from("GruvboxDark")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active_vault: String::from("personal"),
            theme: String::from("GruvboxDark"),
            auto_backup: true,
            max_backups: 10,
            clipboard_timeout: 30,
            refresh_rate: 100,
        }
    }
}

pub fn get_passlock_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".passlock")
}

pub fn get_vaults_dir() -> PathBuf {
    get_passlock_dir().join("vaults")
}

pub fn get_config_path() -> PathBuf {
    get_passlock_dir().join("config.json")
}

pub fn get_vault_path(vault_name: &str) -> PathBuf {
    get_vaults_dir().join(format!("{vault_name}.vault"))
}

pub fn init_passlock_dirs() -> Result<(), Box<dyn std::error::Error>> {
    let passlock_dir = get_passlock_dir();
    let vaults_dir = get_vaults_dir();

    if !passlock_dir.exists() {
        fs::create_dir_all(&passlock_dir)?;
        println!("[✔] Created PassLock directory: {}", passlock_dir.display());
    }

    if !vaults_dir.exists() {
        fs::create_dir_all(&vaults_dir)?;
        println!("[✔] Created vaults directory: {}", vaults_dir.display());
    }

    let home = dirs::home_dir().expect("Could not find home directory");
    let old_vault = home.join(".passlock.vault");
    if old_vault.exists() {
        let new_vault = get_vault_path("personal");
        if !new_vault.exists() {
            fs::copy(&old_vault, &new_vault)?;
            println!("[✔] Migrated old vault to: {}", new_vault.display());
            println!("[!] You can delete the old vault: {}", old_vault.display());
        }
    }

    Ok(())
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();

    if config_path.exists() {
        let config_str = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&config_str)?;

        let needs_update = !config_str.contains("\"theme\"");

        if needs_update {
            save_config(&config)?;
        }

        Ok(config)
    } else {
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let config_str = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_str)?;
    Ok(())
}

pub fn list_vaults() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let vaults_dir = get_vaults_dir();

    if !vaults_dir.exists() {
        return Ok(Vec::new());
    }

    let mut vaults = Vec::new();

    for entry in fs::read_dir(vaults_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("vault") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                vaults.push(name.to_string());
            }
        }
    }

    vaults.sort();
    Ok(vaults)
}

pub fn vault_exists(vault_name: &str) -> bool {
    get_vault_path(vault_name).exists()
}

pub fn delete_vault(vault_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vault_path = get_vault_path(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{vault_name}' does not exist").into());
    }

    fs::remove_file(&vault_path)?;

    let backup_dir = crate::backup::gvback_dir(vault_name);
    if backup_dir.exists() {
        fs::remove_dir_all(&backup_dir)?;
    }

    println!("[✔] Deleted vault: {vault_name}");
    println!("[✔] Deleted backups for: {vault_name}");

    Ok(())
}

pub fn rename_vault(old_name: &str, new_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let old_path = get_vault_path(old_name);
    let new_path = get_vault_path(new_name);

    if !old_path.exists() {
        return Err(format!("Vault '{old_name}' does not exist").into());
    }

    if new_path.exists() {
        return Err(format!("Vault '{new_name}' already exists").into());
    }

    fs::rename(&old_path, &new_path)?;

    let old_backup_dir = crate::backup::gvback_dir(old_name);
    let new_backup_dir = crate::backup::gvback_dir(new_name);
    if old_backup_dir.exists() {
        fs::rename(&old_backup_dir, &new_backup_dir)?;
    }

    println!("[✔] Renamed vault: {old_name} → {new_name}");

    Ok(())
}
