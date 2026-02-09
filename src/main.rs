mod crypto;
mod models;
mod storage;
mod vault_ffi;
mod ui;
mod daemon;
mod gui;
mod autofill;

pub use models::{get_timestamp, generate_uuid};

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "create" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock create <password>");
                std::process::exit(1);
            }
            crypto::ct_vt(&args[2])?;
            println!("Vault created successfully!");
        }
        "unlock" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock unlock <password>");
                std::process::exit(1);
            }
            let vault = storage::ld_vt(&args[2])?;
            println!("Vault unlocked! {} entries found", vault.e.len());
        }
        "sync" => {
            if args.len() < 3 {
                eprintln!("Usage: passlock sync <password>");
                std::process::exit(1);
            }
            let mut vault = storage::ld_vt(&args[2])?;
            storage::svv(&mut vault, &args[2])?;
            println!("Vault synced successfully!");
        }
        "tui" => {
            ui::run_tui()?;
        }
        "daemon" => {
            handle_daemon_command(&args[2..]).await?;
        }
        "version" => {
            println!("PassLock v{}", env!("CARGO_PKG_VERSION"));
        }
        "help" => {
            print_usage();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn handle_daemon_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    use crate::daemon::{PassLockDaemon, HotkeyManager};
    use crate::daemon::hotkeys::HotkeyAction;
    use crate::daemon::window::get_active_window_context;
    use crate::gui::{show_capture_dialog, show_select_dialog};
    use crate::autofill::type_credentials;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    if args.is_empty() {
        println!("Usage: passlock daemon [start|stop|status]");
        println!("");
        println!("Commands:");
        println!("  start   - Start the daemon with hotkey support");
        println!("  stop    - Stop the running daemon");
        println!("  status  - Show daemon status");
        return Ok(());
    }

    match args[0].as_str() {
        "start" => {
            println!("Starting PassLock daemon...");
            println!("");
            
            if !storage::vt_exi() {
                eprintln!("No vault found!");
                eprintln!("   Create one first: passlock create <password>");
                std::process::exit(1);
            }
            
            let daemon = Arc::new(Mutex::new(PassLockDaemon::new()));
            
            println!("Enter master password to unlock vault:");
            let password = read_password()?;
            
            {
                let daemon_guard = daemon.lock().await;
                daemon_guard.unlock_vault(&password).await?;
            }
            
            {
                let daemon_guard = daemon.lock().await;
                daemon_guard.start().await?;
            }
            
            println!("");
            println!("Daemon is running");
            println!("");
            println!("Hotkeys:");
            println!("   Ctrl+Shift+P - Capture password");
            println!("   Ctrl+Shift+A - Auto-fill password");
            println!("   Ctrl+Shift+L - Lock vault");
            println!("");
            println!("Press Ctrl+C to stop the daemon");
            println!("");
            
            let hotkey_manager = HotkeyManager::new()?;
            let (tx, mut rx) = HotkeyManager::create_event_channel();
            
            let daemon_clone = Arc::clone(&daemon);
            tokio::spawn(async move {
                let _ = hotkey_manager.listen(tx).await;
            });
            
            loop {
                if let Some(action) = rx.recv().await {
                    match action {
                        HotkeyAction::Capture => {
                            println!("Capture triggered!");
                            
                            if let Ok(context) = get_active_window_context() {
                                println!("   Context: {} ({})", context.suggested_name, context.app_name);
                                
                                if let Some(entry) = show_capture_dialog(context) {
                                    let daemon_guard = daemon_clone.lock().await;
                                    match daemon_guard.save_entry(entry).await {
                                        Ok(_) => println!("Password saved"),
                                        Err(e) => eprintln!("Failed to save: {}", e),
                                    }
                                }
                            } else {
                                eprintln!("Failed to get window context");
                            }
                        }
                        HotkeyAction::AutoFill => {
                            println!("Auto-fill triggered!");
                            
                            if let Ok(context) = get_active_window_context() {
                                println!("   Context: {} ({})", context.suggested_name, context.app_name);
                                
                                let daemon_guard = daemon_clone.lock().await;
                                let entries = daemon_guard.search_entries(&context.suggested_name).await;
                                drop(daemon_guard);
                                
                                if entries.is_empty() {
                                    println!("No matching passwords found");
                                } else if entries.len() == 1 {
                                    println!("Auto-filling: {}", entries[0].n);
                                    match type_credentials(&entries[0]) {
                                        Ok(_) => println!("Credentials typed!"),
                                        Err(e) => eprintln!("Auto-fill failed: {}", e),
                                    }
                                } else {
                                    println!("{} matches found, showing selection...", entries.len());
                                    if let Some(selected) = show_select_dialog(entries) {
                                        println!("Auto-filling: {}", selected.n);
                                        match type_credentials(&selected) {
                                            Ok(_) => println!("Credentials typed!"),
                                            Err(e) => eprintln!("Auto-fill failed: {}", e),
                                        }
                                    }
                                }
                            } else {
                                eprintln!("Failed to get window context");
                            }
                        }
                        HotkeyAction::Lock => {
                            println!("Lock triggered!");
                            let daemon_guard = daemon_clone.lock().await;
                            match daemon_guard.lock_vault().await {
                                Ok(_) => println!("Vault locked!"),
                                Err(e) => eprintln!("Failed to lock: {}", e),
                            }
                        }
                    }
                }
                
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        }
        "stop" => {
            println!("Stopping PassLock daemon...");
            println!("Use Ctrl+C to stop the daemon for now");
            println!("(Proper stop command coming in next version)");
        }
        "status" => {
            println!("PassLock Daemon Status");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Status: Check if process is running manually");
            println!("(Proper status checking coming in next version)");
            println!("");
            println!("Tip: Use 'ps aux | grep passlock' to check if running");
        }
        _ => {
            println!("Unknown daemon command: {}", args[0]);
            println!("");
            println!("Available commands:");
            println!("  start   - Start the daemon");
            println!("  stop    - Stop the daemon");
            println!("  status  - Show status");
        }
    }

    Ok(())
}

fn read_password() -> Result<String, Box<dyn std::error::Error>> {
    use std::io::{self, Write};
    print!("Password: ");
    io::stdout().flush()?;
    
    let password = rpassword::read_password()?;
    Ok(password)
}

fn print_usage() {
    println!("PassLock v{} - Secure Password Manager", env!("CARGO_PKG_VERSION"));
    println!("");
    println!("USAGE:");
    println!("  passlock <COMMAND> [OPTIONS]");
    println!("");
    println!("COMMANDS:");
    println!("  create <password>   Create a new vault");
    println!("  unlock <password>   Unlock and verify vault");
    println!("  sync <password>     Sync vault changes");
    println!("  tui                 Launch TUI interface");
    println!("  daemon <cmd>        Daemon management");
    println!("  version             Show version");
    println!("  help                Show this help");
    println!("");
    println!("DAEMON COMMANDS:");
    println!("  daemon start        Start daemon with hotkeys");
    println!("  daemon stop         Stop running daemon");
    println!("  daemon status       Show daemon status");
    println!("");
    println!("HOTKEYS (when daemon running):");
    println!("  Ctrl+Shift+P        Capture password");
    println!("  Ctrl+Shift+A        Auto-fill password");
    println!("  Ctrl+Shift+L        Lock vault");
    println!("");
    println!("EXAMPLES:");
    println!("  passlock create mySecurePassword123");
    println!("  passlock tui");
    println!("  passlock daemon start");
    println!("");
    println!("For more info: https://github.com/hachimamma/passlock");
}