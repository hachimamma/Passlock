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
        process_command(&args, &active_vault)?;
    } else {
        print_usage();
    }

    crypto::cleanup();

    Ok(())
}

/// Process command line arguments
///
/// # Errors
/// Returns an error if the command fails to execute.
fn process_command(args: &[String], active_vault: &str) -> Result<(), Box<dyn std::error::Error>> {
    match args[1].as_str() {
        "unlock" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock unlock <password>");
                std::process::exit(1);
            }
            unlock_vault(&args[2])
        }
        "sync" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock sync <password>");
                std::process::exit(1);
            }
            sync_vault(&args[2])
        }
        "backup" => handle_backup_cmd(&args[2..]),
        "vault" => handle_vault_cmd(&args[2..]),
        "export" => handle_export_cmd(args, active_vault),
        "import" => handle_import_cmd(args, active_vault),
        "export-csv" => handle_export_csv_cmd(args, active_vault),
        "export-json" => handle_export_json_cmd(args, active_vault),
        "import-csv" => handle_import_csv_cmd(args, active_vault),
        "import-json" => handle_import_json_cmd(args, active_vault),
        "info" => {
            handle_icmd(&args[2..]);
            Ok(())
        }
        "version" => {
            println!("PassLock v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        "help" => {
            print_usage();
            Ok(())
        }
        "tui" => ui::run_tui(),
        _ => {
            println!("Unknown command: {}", args[1]);
            println!();
            print_usage();
            std::process::exit(1);
        }
    }
}

/// Handle export command
fn handle_export_cmd(
    args: &[String],
    active_vault: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    backup::export_vault(active_vault, password, output_file)
}

/// Handle import command
fn handle_import_cmd(
    args: &[String],
    active_vault: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    backup::import_vault(active_vault, password, input_file)
}

/// Handle export CSV command
fn handle_export_csv_cmd(
    args: &[String],
    active_vault: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 4 {
        eprintln!("Usage: passlock export-csv <password> <output-file.csv>");
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
    backup::export_csv(active_vault, password, output_file)
}

/// Handle export JSON command
fn handle_export_json_cmd(
    args: &[String],
    active_vault: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    backup::export_json(active_vault, password, output_file)
}

/// Handle import CSV command
fn handle_import_csv_cmd(
    args: &[String],
    active_vault: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    backup::import_csv(active_vault, password, input_file)
}

/// Handle import JSON command
fn handle_import_json_cmd(
    args: &[String],
    active_vault: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    backup::import_json(active_vault, password, input_file)
}

/// Unlock vault command
fn unlock_vault(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vault = storage::ld_vt(password)?;
    println!("[✔] Vault unlocked successfully.");
    println!("[✔] Found {} entries", vault.e.len());
    Ok(())
}

/// Sync vault command
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

/// Automatically backup vault after TUI closes
/// Called by TUI when exiting
/// Only creates backup if vault was modified
///
/// # Errors
/// Returns an error if the backup creation fails.
pub fn auto_back() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::load_config().unwrap_or_default();

    if !cfg.auto_backup {
        return Ok(());
    }

    if !config::vault_exists(&cfg.active_vault) {
        return Ok(());
    }

    println!("[...] Auto-backup in progress...");
    let (backup_name, was_created) =
        backup::create_backup(&cfg.active_vault, cfg.max_backups, false)?;

    if was_created {
        println!("[✔] Auto-backup completed: {backup_name}");
    }

    Ok(())
}

/// Print backup help
fn print_backup_help() {
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
}

/// Handle backup subcommands
fn handle_backup_cmd(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        print_backup_help();
        return Ok(());
    }

    match args[0].as_str() {
        "create" => handle_backup_create(args),
        "list" => handle_backup_list(),
        "restore" => handle_backup_restore(args),
        _ => {
            println!("Unknown backup subcommand: {}", args[0]);
            println!();
            println!("Available subcommands: create, list, restore");
            Ok(())
        }
    }
}

/// Handle backup create subcommand
fn handle_backup_create(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}

/// Handle backup list subcommand
fn handle_backup_list() -> Result<(), Box<dyn std::error::Error>> {
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
        println!("  passlock backup restore <backup-name> <password>");
    }
    Ok(())
}

/// Handle backup restore subcommand
fn handle_backup_restore(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        eprintln!("Usage: passlock backup restore <backup-name|latest> <password>");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  passlock backup restore backup_2026-02-28_10-30-45.vault myPassword123");
        eprintln!("  passlock backup restore latest myPassword123");
        std::process::exit(1);
    }

    let cfg = config::load_config().unwrap_or_default();

    if args[1] == "latest" {
        handle_restore_latest(args, &cfg)
    } else {
        handle_restore_named(args, &cfg)
    }
}

/// Handle restore latest backup
fn handle_restore_latest(
    args: &[String],
    cfg: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("[i] Restoring latest backup: {latest_backup}");
    backup::restore_backup(&cfg.active_vault, latest_backup, password)?;
    println!("[✔] Vault restored to latest backup successfully");
    Ok(())
}

/// Handle restore named backup
fn handle_restore_named(
    args: &[String],
    cfg: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        eprintln!("Usage: passlock backup restore <backup-name> <password>");
        std::process::exit(1);
    }
    let backup_name = &args[1];
    let password = &args[2];

    backup::restore_backup(&cfg.active_vault, backup_name, password)?;
    println!("[✔] Backup restored successfully");
    Ok(())
}

/// Print vault help
fn print_vault_help() {
    println!("Vault Management Commands:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Usage: passlock vault <subcommand>");
    println!();
    println!("Subcommands:");
    println!("  create <name> <password>       Create new vault");
    println!("  list                           List all vaults");
    println!("  use <name>                      Set active vault");
    println!("  info                           Show current vault info");
    println!("  delete <name>                   Delete vault");
    println!("  rename <old> <new>             Rename vault");
    println!();
    println!("Examples:");
    println!("  passlock vault create personal myPass123");
    println!("  passlock vault create work workPass456");
    println!("  passlock vault list");
    println!("  passlock vault use work");
}

/// Handle vault management subcommands
fn handle_vault_cmd(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        print_vault_help();
        return Ok(());
    }

    match args[0].as_str() {
        "create" => handle_vault_create(args),
        "list" => handle_vault_list(),
        "use" => handle_vault_use(args),
        "info" => handle_vault_info(),
        "delete" => handle_vault_delete(args),
        "rename" => handle_vault_rename(args),
        _ => {
            println!("Unknown vault subcommand: {}", args[0]);
            println!();
            println!("Available subcommands: create, list, use, info, delete, rename");
            Ok(())
        }
    }
}

/// Handle vault create subcommand
fn handle_vault_create(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
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
        return Err(format!("Vault '{vault_name}' already exists").into());
    }

    let salt = crypto::gen_salt();
    let vault = Vault::new(salt);

    storage::save_vault_to(vault_name, &vault, password)?;

    println!("[✔] Vault '{vault_name}' created successfully");
    println!("[i] Backups will be created automatically as you add passwords");

    let mut cfg = config::load_config()?;
    cfg.active_vault.clone_from(vault_name);
    config::save_config(&cfg)?;
    println!("[✔] Vault '{vault_name}' set as active");
    Ok(())
}

/// Handle vault list subcommand
fn handle_vault_list() -> Result<(), Box<dyn std::error::Error>> {
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
            let marker = if vault_name == cfg.active_vault {
                " (active)"
            } else {
                ""
            };
            println!("  • {vault_name}{marker}");
        }
        println!();
        println!("Switch vault with: passlock vault use <name>");
    }
    Ok(())
}

/// Handle vault use subcommand
fn handle_vault_use(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        eprintln!("Usage: passlock vault use <name>");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  passlock vault use work");
        std::process::exit(1);
    }
    let vault_name = &args[1];

    if !config::vault_exists(vault_name) {
        return Err(format!("Vault '{vault_name}' does not exist").into());
    }

    let mut cfg = config::load_config()?;
    cfg.active_vault.clone_from(vault_name);
    config::save_config(&cfg)?;

    println!("[✔] Active vault set to: {vault_name}");
    Ok(())
}

/// Handle vault info subcommand
fn handle_vault_info() -> Result<(), Box<dyn std::error::Error>> {
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
                println!("Vault size: {size_kb} KB");
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
    Ok(())
}

/// Handle vault delete subcommand
fn handle_vault_delete(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        eprintln!("Usage: passlock vault delete <name>");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  passlock vault delete old-vault");
        std::process::exit(1);
    }
    let vault_name = &args[1];

    if !config::vault_exists(vault_name) {
        return Err(format!("Vault '{vault_name}' does not exist").into());
    }

    let mut cfg = config::load_config()?;
    let is_active = vault_name == &cfg.active_vault;

    if is_active {
        println!("WARNING: You are deleting the ACTIVE vault.");
    }

    println!("This will delete vault '{vault_name}' and all its backups.");
    println!("Are you sure? (y/n)");

    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    if line.trim().to_lowercase() == "y" {
        config::delete_vault(vault_name)?;

        if is_active {
            let remaining_vaults = config::list_vaults()?;

            if remaining_vaults.is_empty() {
                cfg.active_vault.clear();
                config::save_config(&cfg)?;
                println!("[✔] Vault deleted successfully");
                println!("[i] No vaults remaining. Create a new vault with:");
                println!("    passlock vault create <name> <password>");
            } else {
                cfg.active_vault.clear();
                config::save_config(&cfg)?;
                println!("[✔] Vault deleted successfully");
                println!("[i] No active vault selected. Available vaults:");
                for v in &remaining_vaults {
                    println!("    • {v}");
                }
                println!();
                println!("Set active vault with: passlock vault use <name>");
            }
        } else {
            println!("[✔] Vault '{vault_name}' deleted successfully");
        }
    } else {
        println!("[!] Deletion cancelled");
    }
    Ok(())
}

/// Handle vault rename subcommand
fn handle_vault_rename(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        eprintln!("Usage: passlock vault rename <old-name> <new-name>");
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
        cfg.active_vault.clone_from(new_name);
        config::save_config(&cfg)?;
    }
    Ok(())
}

/// Handle info command
fn handle_icmd(args: &[String]) {
    if args.is_empty() || args[0] == "cpu" {
        print_system_info();
    } else {
        println!("Unknown info command: {}", args[0]);
        println!("Available: info, info cpu");
    }
}

/// Print system information
fn print_system_info() {
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

    print_vault_status();
    println!();
    println!("Version: PassLock v{}", env!("CARGO_PKG_VERSION"));
}

/// Print vault status
fn print_vault_status() {
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
        println!(
            "  Backups: {} available (vault: {})",
            backups.len(),
            cfg.active_vault
        );
    } else {
        println!("Vault Status:");
        println!("  No vault found");
        println!("  Create one with: passlock create <password>");
    }
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
    println!("  vault create <name> <password>       Create new vault");
    println!("  vault list                       List all vaults");
    println!("  vault use <name>                 Set active vault");
    println!("  vault info                       Show vault info");
    println!("  vault delete <name>              Delete vault");
    println!("  vault rename <old> <new>         Rename vault");
    println!();
    println!("BACKUP COMMANDS (Encrypted):");
    println!("  backup create <password>         Create manual backup");
    println!("  backup list                      List all backups");
    println!("  backup restore <name|latest> <p> Restore from backup");
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
    println!("For more info: https://github.com/hachimamma/Passlock");
}
