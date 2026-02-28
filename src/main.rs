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
    println!("COMMANDS:");
    println!("  create <password>   Create a new vault");
    println!("  unlock <password>   Unlock and verify vault");
    println!("  sync <password>     Sync vault changes");
    println!("  tui                 Launch TUI interface");
    println!("  info [cpu]          Show system information");
    println!("  version             Show version");
    println!("  help                Show this help");
    println!();
    println!("EXAMPLES:");
    println!("  passlock create mySecurePassword123");
    println!("  passlock tui");
    println!("  passlock info cpu");
    println!();
    println!("For more info: https://github.com/hachimamma/Passlock");
}