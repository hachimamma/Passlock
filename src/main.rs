mod crypto;
mod models;
mod storage;
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
            _ => {
                ui::run_tui()?;
            }
        }
    } else {
        ui::run_tui()?;
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
    let _vault = storage::ld_vt(password)?;
    println!("[✔] Vault unlocked successfully.");
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
