mod backup;
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

    backup::init_backup_system()?;

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "create" => {
                if args.len() < 3 {
                    eprintln!("Usage: passlock create <password>");
                    std::process::exit(1);
                }
                let password = &args[2];
                create_vault(password)?;
            }
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
            "export" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock export <password> <output-file>");
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
                backup::export_vault(password, output_file)?;
            }
            "import" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock import <password> <input-file>");
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
                backup::import_vault(password, input_file)?;
            }
            "export-csv" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock export-csv <password> <output-file.csv>");
                    eprintln!();
                    eprintln!("WARNING: Creates UNENCRYPTED CSV with plaintext passwords!");
                    eprintln!(
                        "Use for importing to other password managers (LastPass, Bitwarden, etc.)"
                    );
                    eprintln!();
                    eprintln!("Example:");
                    eprintln!("  passlock export-csv myPass123 ~/passwords.csv");
                    std::process::exit(1);
                }
                let password = &args[2];
                let output_file = &args[3];
                backup::export_to_csv(password, output_file)?;
            }
            "export-json" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock export-json <password> <output-file.json>");
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
                backup::export_to_json(password, output_file)?;
            }
            "import-csv" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock import-csv <password> <input-file.csv>");
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
                backup::import_from_csv(password, input_file)?;
            }
            "import-json" => {
                if args.len() < 4 {
                    eprintln!("Usage: passlock import-json <password> <input-file.json>");
                    eprintln!();
                    eprintln!("Import passwords from JSON file.");
                    eprintln!();
                    eprintln!("Example:");
                    eprintln!("  passlock import-json myPass123 ~/passwords.json");
                    std::process::exit(1);
                }
                let password = &args[2];
                let input_file = &args[3];
                backup::import_from_json(password, input_file)?;
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

fn create_vault(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    if storage::vt_exi() {
        return Err("Vault already exists".into());
    }

    let salt = crypto::gen_salt();
    let vault = Vault::new(salt);

    storage::svv(&vault, password)?;

    println!("[✔] Vault created successfully.");

    println!("[...] Creating initial backup...");
    backup::create_backup("default", 10)?;

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
    backup::create_backup("default", 10)?;

    Ok(())
}

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
        println!("  restore <backup-name> <password>  Restore from backup");
        println!();
        println!("Examples:");
        println!("  passlock backup create myPassword123");
        println!("  passlock backup list");
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

            backup::create_backup("default", 10)?;
            println!("[✔] Manual backup created successfully");
        }
        "list" => {
            let backups = backup::list_backups("default")?;

            if backups.is_empty() {
                println!("[!] No backups found");
                println!();
                println!("Create your first backup with:");
                println!("  passlock backup create <password>");
            } else {
                println!("Available Backups:");
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
                println!("  passlock backup restore <backup-name> <password>");
            }
        }
        "restore" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock backup restore <backup-name> <password>");
                eprintln!();
                eprintln!("Example:");
                eprintln!(
                    "  passlock backup restore backup_2026-02-28_10-30-45.vault myPassword123"
                );
                std::process::exit(1);
            }
            let backup_name = &args[1];
            let password = &args[2];

            backup::restore_backup("default", backup_name, password)?;
            println!("[✔] Backup restored successfully");
        }
        _ => {
            println!("Unknown backup subcommand: {}", args[0]);
            println!();
            println!("Available subcommands: create, list, restore");
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

            let backups = backup::list_backups("default").unwrap_or_else(|_| Vec::new());
            println!("  Backups: {} available", backups.len());
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
    println!("  create <password>                Create a new vault");
    println!("  unlock <password>                Unlock and verify vault");
    println!("  sync <password>                  Sync vault changes");
    println!("  tui                              Launch TUI interface");
    println!();
    println!("BACKUP COMMANDS (Encrypted):");
    println!("  backup create <password>         Create manual backup");
    println!("  backup list                      List all backups");
    println!("  backup restore <n> <pass>     Restore from backup");
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
    println!("EXAMPLES:");
    println!();
    println!("  # Create and use vault");
    println!("  passlock create mySecurePassword123");
    println!("  passlock tui");
    println!();
    println!("  # Backup to any location (works everywhere!)");
    println!("  passlock export myPass123 ~/Documents/backup.vault");
    println!("  passlock export myPass123 /media/usb/backup.vault");
    println!("  passlock export myPass123 ~/Dropbox/backup.vault");
    println!();
    println!("  # Migrate from LastPass/Bitwarden");
    println!("  passlock import-csv myPass123 ~/lastpass-export.csv");
    println!();
    println!("  # Migrate to another password manager");
    println!("  passlock export-csv myPass123 ~/passwords.csv");
    println!("  # (Then import into other PM and DELETE the CSV!)");
    println!();
    println!("For more info: https://github.com/hachimamma/Passlock");
}
