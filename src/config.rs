use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration for PassLock
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Currently active vault name
    pub active_vault: String,
    /// Auto-backup on every save (if false, only manual backups)
    pub auto_backup: bool,
    /// Maximum number of backups to keep per vault
    pub max_backups: usize,
    /// Clipboard timeout in seconds
    pub clipboard_timeout: u64,
    /// UI refresh rate in milliseconds
    pub refresh_rate: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active_vault: String::from("personal"),
            auto_backup: true,
            max_backups: 10,
            clipboard_timeout: 30,
            refresh_rate: 100,
        }
    }
}

/// Get the PassLock root directory: ~/.passlock/
pub fn get_passlock_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".passlock")
}

/// Get the vaults directory: ~/.passlock/vaults/
pub fn get_vaults_dir() -> PathBuf {
    get_passlock_dir().join("vaults")
}

/// Get the config file path: ~/.passlock/config.json
pub fn get_config_path() -> PathBuf {
    get_passlock_dir().join("config.json")
}

/// Get the path to a specific vault file
pub fn get_vault_path(vault_name: &str) -> PathBuf {
    get_vaults_dir().join(format!("{}.vault", vault_name))
}

/// Initialize the PassLock directory structure
pub fn init_passlock_dirs() -> Result<(), Box<dyn std::error::Error>> {
    let passlock_dir = get_passlock_dir();
    let vaults_dir = get_vaults_dir();
    
    // Create main directory
    if !passlock_dir.exists() {
        fs::create_dir_all(&passlock_dir)?;
        println!("[✔] Created PassLock directory: {}", passlock_dir.display());
    }
    
    // Create vaults directory
    if !vaults_dir.exists() {
        fs::create_dir_all(&vaults_dir)?;
        println!("[✔] Created vaults directory: {}", vaults_dir.display());
    }
    
    // Check for old vault file and migrate it
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

/// Load configuration from file
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    } else {
        // Create default config
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let config_str = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_str)?;
    Ok(())
}

/// List all available vaults
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

/// Check if a vault exists
pub fn vault_exists(vault_name: &str) -> bool {
    get_vault_path(vault_name).exists()
}

/// Delete a vault and its backups
pub fn delete_vault(vault_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vault_path = get_vault_path(vault_name);
    
    if !vault_path.exists() {
        return Err(format!("Vault '{}' does not exist", vault_name).into());
    }
    
    // Delete vault file
    fs::remove_file(&vault_path)?;
    
    // Delete backups
    let backup_dir = crate::backup::get_vault_backup_dir(vault_name);
    if backup_dir.exists() {
        fs::remove_dir_all(&backup_dir)?;
    }
    
    println!("[✔] Deleted vault: {}", vault_name);
    println!("[✔] Deleted backups for: {}", vault_name);
    
    Ok(())
}

/// Rename a vault
pub fn rename_vault(old_name: &str, new_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let old_path = get_vault_path(old_name);
    let new_path = get_vault_path(new_name);
    
    if !old_path.exists() {
        return Err(format!("Vault '{}' does not exist", old_name).into());
    }
    
    if new_path.exists() {
        return Err(format!("Vault '{}' already exists", new_name).into());
    }
    
    fs::rename(&old_path, &new_path)?;
    
    // Rename backup directory
    let old_backup_dir = crate::backup::get_vault_backup_dir(old_name);
    let new_backup_dir = crate::backup::get_vault_backup_dir(new_name);
    if old_backup_dir.exists() {
        fs::rename(&old_backup_dir, &new_backup_dir)?;
    }
    
    println!("[✔] Renamed vault: {} → {}", old_name, new_name);
    
    Ok(())
}