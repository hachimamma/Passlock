mod backup;
mod config;
mod crypto;
mod models;
mod storage;
mod totp;
mod ui;
mod vault_ffi;

use models::Vault;
use std::env;

/// Generates a UUID string.
///
/// # Panics
/// Panics if the system time is before `UNIX_EPOCH`.
#[must_use]
pub fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{now}")
}

/// Gets the current timestamp.
///
/// # Panics
/// Panics if the system time is before `UNIX_EPOCH`.
#[must_use]
pub fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crypto::init_crypto()?;
    
    config::init_passlock_dirs()?;
    backup::init_bsys()?;
    
    let cfg = config::load_config().unwrap_or_default();
    let active_vault = cfg.active_vault.clone();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "unlock" => {
                if args.len() < 3 {
                    eprintln!("Usage: passlock unlock <password>");
                    std::process::exit(1);
                }
                let password = &args[2];
                unlock_vault(password)?;
            }
            "sync" => {
                if args.len() < 3 {
                    eprintln!("Usage: passlock sync <password>");
                    std::process::exit(1);
                }
                let password = &args[2];
                sync_vault(password)?;
            }
            "backup" => {
                handle_backup_cmd(&args[2..])?;
            }
            "vault" => {
                handle_vault_cmd(&args[2..])?;
            }
            "export" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock export <password> <output_file>");
                    eprintln!();
                    eprintln!("Export encrypted vault for backup/transfer.");
                    eprintln!("Works with ANY path on ANY device!");
                    eprintln!();
                    eprintln!("Examples:");
                    eprintln!("  passlock export myPass123 ~/Documents/backup.vault");
                    eprintln!("  passlock export myPass123 /media/usb/backup.vault");
                    eprintln!("  passlock export myPass123 ~/Dropbox/backup.vault");
                    std::process::exit(1);
                }
                let password = &args[2];
                let output_file = &args[3];
                backup::export_vault(&active_vault, password, output_file)?;
            }
            "import" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock import <password> <input_file>");
                    eprintln!();
                    eprintln!("Import encrypted vault from backup.");
                    eprintln!("Works with ANY path on ANY device!");
                    eprintln!();
                    eprintln!("Examples:");
                    eprintln!("  passlock import myPass123 ~/Documents/backup.vault");
                    eprintln!("  passlock import myPass123 /media/usb/backup.vault");
                    std::process::exit(1);
                }
                let password = &args[2];
                let input_file = &args[3];
                backup::import_vault(&active_vault, password, input_file)?;
            }
            "export-csv" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock export_csv <password> <output_file.csv>");
                    eprintln!();
                    eprintln!("WARNING: Creates UNENCRYPTED CSV with plaintext passwords!");
                    eprintln!("Use for importing to other password managers (LastPass, Bitwarden, etc.)");
                    eprintln!();
                    eprintln!("Example:");
                    eprintln!("  passlock export-csv myPass123 ~/passwords.csv");
                    std::process::exit(1);
                }
                let password = &args[2];
                let output_file = &args[3];
                backup::export_csv(&active_vault, password, output_file)?;
            }
            "export-json" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock export_json <password> <output_file.json>");
                    eprintln!();
                    eprintln!("WARNING: Creates UNENCRYPTED JSON with plaintext passwords!");
                    eprintln!("Use for importing to other password managers.");
                    eprintln!();
                    eprintln!("Example:");
                    eprintln!("  passlock export-json myPass123 ~/passwords.json");
                    std::process::exit(1);
                }
                let password = &args[2];
                let output_file = &args[3];
                backup::export_json(&active_vault, password, output_file)?;
            }
            "import-csv" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock import-csv <password> <input_file.csv>");
                    eprintln!();
                    eprintln!("Import passwords from CSV file (LastPass, Bitwarden, etc.)");
                    eprintln!("Expected format: name,username,password,url,notes,tags,2fa_secret");
                    eprintln!();
                    eprintln!("Example:");
                    eprintln!("  passlock import-csv myPass123 ~/lastpass-export.csv");
                    std::process::exit(1);
                }
                let password = &args[2];
                let input_file = &args[3];
                backup::import_csv(&active_vault, password, input_file)?;
            }
            "import-json" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock import_json <password> <input_file.json>");
                    eprintln!();
                    eprintln!("Import passwords from JSON file.");
                    eprintln!();
                    eprintln!("Example:");
                    eprintln!("  passlock import-json myPass123 ~/passwords.json");
                    std::process::exit(1);
                }
                let password = &args[2];
                let input_file = &args[3];
                backup::import_json(&active_vault, password, input_file)?;
            }
            "info" => {
                handle_icmd(&args[2..])?;
            }
            "version" => {
                println!("PassLock v{}", env!("CARGO_PKG_VERSION"));
            }
            "help" => {
                print_usage();
            }
            "tui" => {
                ui::run_tui()?;
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                println!();
                print_usage();
                std::process::exit(1);
            }
        }
    } else {
        print_usage();
    }

    crypto::cleanup();

    Ok(())
}

fn unlock_vault(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vault = storage::ld_vt(password)?;
    println!("[✔] Vault unlocked successfully.");
    println!("[✔] Found {} entries", vault.e.len());
    Ok(())
}

fn sync_vault(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().expect("no home");
    let temp_path = home.join(".passlock.temp");

    if !temp_path.exists() {
        return Err("[X] No temp file to sync.".into());
    }

    let temp_data = std::fs::read_to_string(&temp_path)?;
    let vault: Vault = serde_json::from_str(&temp_data)?;

    storage::svv(&vault, password)?;

    println!("[✔] Vault synced successfully.");
    
    println!("[...] Creating backup...");
    let cfg = config::load_config().unwrap_or_default();
    backup::create_backup(&cfg.active_vault, cfg.max_backups, false)?;
    
    Ok(())
}

/// Automatically backups vault after TUI closes
/// Called by TUI when exiting
/// Only creates backup if vault was modified
pub fn auto_back() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::load_config().unwrap_or_default();
    
    if !cfg.auto_backup {
        return Ok(());
    }
    
    if !config::vault_exists(&cfg.active_vault) {
        return Ok(());
    }
    
    println!("auto backup in progress");
    let (backup_name, was_created) = backup::create_backup(&cfg.active_vault, cfg.max_backups, false)?;
    
    if was_created {
        println!("auto backup completed {}", backup_name);
    }
    
    Ok(())
}

/// Handle backup subcommands
fn handle_backup_cmd(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        println!("Backup Commands:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("Usage: passlock backup <subcommand>");
        println!();
        println!("Subcommands:");
        println!("  create <password>              Create manual backup");
        println!("  list                           List all backups");
        println!("  restore <name|latest> <pass>   Restore from backup");
        println!();
        println!("Examples:");
        println!("  passlock backup create myPassword123");
        println!("  passlock backup list");
        println!("  passlock backup restore latest myPassword123");
        println!("  passlock backup restore backup_2026-02-28_10-30-45.vault myPassword123");
        return Ok(());
    }

    match args[0].as_str() {
        "create" => {
            if args.len() < 2 {
                eprintln!("Usage: passlock backup create <password>");
                std::process::exit(1);
            }
            let password = &args[1];
            
            let _vault = storage::ld_vt(password)?;
            println!("[✔] Password verified");
            
            let cfg = config::load_config().unwrap_or_default();
            backup::create_backup(&cfg.active_vault, cfg.max_backups, true)?;
            println!("[✔] Manual backup created successfully");
        }
        "list" => {
            let cfg = config::load_config().unwrap_or_default();
            let backups = backup::ls_backs(&cfg.active_vault)?;
            
            if backups.is_empty() {
                println!("[!] No backups found for vault '{}'", cfg.active_vault);
                println!();
                println!("Create your first backup with:");
                println!("  passlock backup create <password>");
            } else {
                println!("Available Backups for vault '{}':", cfg.active_vault);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!();
                println!("{:<40} {:<12} Created", "Filename", "Size");
                println!("{}", "─".repeat(70));
                
                for (filename, size, created) in &backups {
                    let size_kb = if *size > 1024 {
                        format!("{} KB", *size / 1024)
                    } else {
                        format!("{} bytes", *size)
                    };
                    println!("{filename:<40} {size_kb:<12} {created}");
                }
                
                println!();
                println!("Total backups: {}", backups.len());
                println!();
                println!("Restore a backup with:");
                println!("  passlock backup restore <backup_name> <password>");
            }
        }
        "restore" => {
            if args.len() < 2 {
                eprintln!("Usage: passlock backup restore <backup_name|latest> <password>");
                eprintln!();
                eprintln!("Examples:");
                eprintln!("  passlock backup restore backup_2026-02-28_10-30-45.vault myPassword123");
                eprintln!("  passlock backup restore latest myPassword123");
                std::process::exit(1);
            }
            
            let cfg = config::load_config().unwrap_or_default();
            
            if args[1] == "latest" {
                if args.len() < 3 {
                    eprintln!("Usage: passlock backup restore latest <password>");
                    std::process::exit(1);
                }
                let password = &args[2];
                
                let backups = backup::ls_backs(&cfg.active_vault)?;
                if backups.is_empty() {
                    return Err("No backups found for this vault".into());
                }
                
                let latest_backup = &backups[0].0;
                println!("[i] Restoring latest backup: {}", latest_backup);
                backup::restore_backup(&cfg.active_vault, latest_backup, password)?;
                println!("[✔] Vault restored to latest backup successfully");
            } else {
                if args.len() < 3 {
                    eprintln!("Usage: passlock backup restore <backup_name> <password>");
                    std::process::exit(1);
                }
                let backup_name = &args[1];
                let password = &args[2];
                
                backup::restore_backup(&cfg.active_vault, backup_name, password)?;
                println!("[✔] Backup restored successfully");
            }
        }
        _ => {
            println!("Unknown backup subcommand: {}", args[0]);
            println!();
            println!("Available subcommands: create, list, restore");
        }
    }
    
    Ok(())
}

/// Handle vault management subcommands
fn handle_vault_cmd(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        println!("Vault Management Commands:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("Usage: passlock vault <subcommand>");
        println!();
        println!("Subcommands:");
        println!("  create <name> <password>       Create new vault");
        println!("  list                           List all vaults");
        println!("  use <name>                     Set active vault");
        println!("  info                           Show current vault info");
        println!("  delete <name>                  Delete vault");
        println!("  rename <old> <new>             Rename vault");
        println!();
        println!("Examples:");
        println!("  passlock vault create personal myPass123");
        println!("  passlock vault create work workPass456");
        println!("  passlock vault list");
        println!("  passlock vault use work");
        return Ok(());
    }

    match args[0].as_str() {
        "create" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock vault create <name> <password>");
                eprintln!();
                eprintln!("Example:");
                eprintln!("  passlock vault create personal myPassword123");
                std::process::exit(1);
            }
            let vault_name = &args[1];
            let password = &args[2];
            
            if config::vault_exists(vault_name) {
                return Err(format!("Vault '{}' already exists", vault_name).into());
            }

            let salt = crypto::gen_salt();
            let vault = Vault::new(salt);
            
            storage::save_vault_to(vault_name, &vault, password)?;
            
            println!("[✔] Vault '{}' created successfully", vault_name);
            println!("[i] Backups will be created automatically as you add passwords");
            
            let mut cfg = config::load_config()?;
            cfg.active_vault = vault_name.to_string();
            config::save_config(&cfg)?;
            println!("[✔] Vault '{}' set as active", vault_name);
        }
        "list" => {
            let vaults = config::list_vaults()?;
            let cfg = config::load_config()?;
            
            if vaults.is_empty() {
                println!("[!] No vaults found");
                println!();
                println!("Create your first vault with:");
                println!("  passlock vault create <name> <password>");
            } else {
                println!("Available Vaults:");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!();
                for vault_name in vaults {
                    let marker = if vault_name == cfg.active_vault { " (active)" } else { "" };
                    println!("  • {}{}", vault_name, marker);
                }
                println!();
                println!("Switch vault with: passlock vault use <name>");
            }
        }
        "use" => {
            if args.len() < 2 {
                eprintln!("Usage: passlock vault use <name>");
                eprintln!();
                eprintln!("Example:");
                eprintln!("  passlock vault use work");
                std::process::exit(1);
            }
            let vault_name = &args[1];
            
            if !config::vault_exists(vault_name) {
                return Err(format!("Vault '{}' does not exist", vault_name).into());
            }
            
            let mut cfg = config::load_config()?;
            cfg.active_vault = vault_name.to_string();
            config::save_config(&cfg)?;
            
            println!("[✔] Active vault set to: {}", vault_name);
        }
        "info" => {
            let cfg = config::load_config()?;
            let vaults = config::list_vaults()?;
            
            println!("Vault Information:");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!();
            println!("Active vault: {}", cfg.active_vault);
            println!("Total vaults: {}", vaults.len());
            println!();
            
            if config::vault_exists(&cfg.active_vault) {
                let vault_path = config::get_vault_path(&cfg.active_vault);
                if let Ok(metadata) = std::fs::metadata(&vault_path) {
                    let size_kb = metadata.len() / 1024;
                    if size_kb > 0 {
                        println!("Vault size: {} KB", size_kb);
                    } else {
                        println!("Vault size: {} bytes", metadata.len());
                    }
                }
                
                let backups = backup::ls_backs(&cfg.active_vault).unwrap_or_else(|_| Vec::new());
                println!("Backups: {} available", backups.len());
            } else {
                println!("Active vault '{}' not found!", cfg.active_vault);
            }
            
            println!();
            println!("Configuration:");
            println!("  Auto-backup: {}", cfg.auto_backup);
            println!("  Max backups: {}", cfg.max_backups);
            println!("  Clipboard timeout: {}s", cfg.clipboard_timeout);
            println!("  Refresh rate: {}ms", cfg.refresh_rate);
        }
        "delete" => {
            if args.len() < 2 {
                eprintln!("Usage: passlock vault delete <name>");
                eprintln!();
                eprintln!("Example:");
                eprintln!("  passlock vault delete old-vault");
                std::process::exit(1);
            }
            let vault_name = &args[1];
            
            let cfg = config::load_config()?;
            if vault_name == &cfg.active_vault {
                return Err("Cannot delete active vault! Switch to another vault first.".into());
            }
            
            println!("WARNING: This will delete vault '{}' and all its backups.", vault_name);
            println!("Are you sure? (y/n)");
            
            use std::io::{self, BufRead};
            let stdin = io::stdin();
            let mut line = String::new();
            stdin.lock().read_line(&mut line)?;
            
            if line.trim().to_lowercase() == "y" {
                config::delete_vault(vault_name)?;
            } else {
                println!("[!] Deletion cancelled");
            }
        }
        "rename" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock vault rename <old_name> <new_name>");
                eprintln!();
                eprintln!("Example:");
                eprintln!("  passlock vault rename work company");
                std::process::exit(1);
            }
            let old_name = &args[1];
            let new_name = &args[2];
            
            config::rename_vault(old_name, new_name)?;
            
            let mut cfg = config::load_config()?;
            if cfg.active_vault == *old_name {
                cfg.active_vault = new_name.to_string();
                config::save_config(&cfg)?;
            }
        }
        _ => {
            println!("Unknown vault subcommand: {}", args[0]);
            println!();
            println!("Available subcommands: create, list, use, info, delete, rename");
        }
    }
    
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
fn handle_icmd(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() || args[0] == "cpu" {
        println!("PassLock System Information");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();

        println!("CPU Features:");
        let has_aes_ni = vault_ffi::aes_sup();

        if has_aes_ni {
            println!("  AES-NI: Supported");
            println!("     (Hardware-accelerated AES encryption available)");
        } else {
            println!("  AES-NI: Not supported");
            println!("     (Using ChaCha20-Poly1305 for optimal performance)");
        }
        println!();

        println!("Recommended Cipher:");
        println!("  {}", vault_ffi::get_cipher());
        println!();

        if storage::vt_exi() {
            println!("Vault Status:");
            println!("  Vault exists at: ~/.passlock.vault");

            if let Some(home) = dirs::home_dir() {
                let vault_path = home.join(".passlock.vault");
                if let Ok(metadata) = std::fs::metadata(&vault_path) {
                    let size_kb = metadata.len() / 1024;
                    if size_kb > 0 {
                        println!("  Size: {size_kb} KB");
                    } else {
                        println!("  Size: {} bytes", metadata.len());
                    }
                }
            }
            
            let cfg = config::load_config().unwrap_or_default();
            let backups = backup::ls_backs(&cfg.active_vault).unwrap_or_else(|_| Vec::new());
            println!("  Backups: {} available (vault: {})", backups.len(), cfg.active_vault);
        } else {
            println!("Vault Status:");
            println!("  No vault found");
            println!("  Create one with: passlock create <password>");
        }
        println!();

        println!("Version: PassLock v{}", env!("CARGO_PKG_VERSION"));
    } else {
        println!("Unknown info command: {}", args[0]);
        println!("Available: info, info cpu");
    }

    Ok(())
}

fn print_usage() {
    println!(
        "PassLock v{} - Secure Password Manager",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("USAGE:");
    println!("  passlock <COMMAND> [OPTIONS]");
    println!();
    println!("VAULT COMMANDS:");
    println!("  unlock <password>                Unlock and verify vault");
    println!("  sync <password>                  Sync vault changes");
    println!("  tui                              Launch TUI interface");
    println!();
    println!("VAULT MANAGEMENT:");
    println!("  vault create <n> <pass>       Create new vault");
    println!("  vault list                       List all vaults");
    println!("  vault use <n>                 Set active vault");
    println!("  vault info                       Show vault info");
    println!("  vault delete <n>              Delete vault");
    println!("  vault rename <old> <new>         Rename vault");
    println!();
    println!("BACKUP COMMANDS (Encrypted):");
    println!("  backup create <password>         Create manual backup");
    println!("  backup list                      List all backups");
    println!("  backup restore <name|latest> <password> Restore from backup");
    println!();
    println!("  export <password> <file>         Export vault (encrypted)");
    println!("  import <password> <file>         Import vault (encrypted)");
    println!();
    println!("PASSWORD MANAGER MIGRATION (Plaintext - USE WITH CAUTION):");
    println!("  export-csv <password> <file>     Export to CSV (for other PMs)");
    println!("  export-json <password> <file>    Export to JSON (for other PMs)");
    println!("  import-csv <password> <file>     Import from CSV");
    println!("  import-json <password> <file>    Import from JSON");
    println!();
    println!("INFO COMMANDS:");
    println!("  info [cpu]                       Show system information");
    println!("  version                          Show version");
    println!("  help                             Show this help");
    println!();
    println!("For more info: https://github.com/hachimamma/Passlock/blob/main/README.md");
}