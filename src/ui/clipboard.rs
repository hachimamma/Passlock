use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static CLEAR_FLAG: std::sync::OnceLock<Arc<Mutex<bool>>> = std::sync::OnceLock::new();

fn get_flag() -> Arc<Mutex<bool>> {
    CLEAR_FLAG
        .get_or_init(|| Arc::new(Mutex::new(false)))
        .clone()
}

pub struct ClipboardResult {
    pub success: bool,
    pub message: String,
    pub expires_at: u64,
}

pub fn copy_with_timeout(text: &str, timeout_secs: u64) -> ClipboardResult {
    if let Ok(mut flag) = get_flag().lock() {
        *flag = true;
    }

    std::thread::sleep(Duration::from_millis(50));

    let result = Command::new("wl-copy").stdin(Stdio::piped()).spawn();

    match result {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                if let Err(e) = stdin.write_all(text.as_bytes()) {
                    return ClipboardResult {
                        success: false,
                        message: format!("Write error: {e}"),
                        expires_at: 0,
                    };
                }
                drop(stdin);

                match child.wait() {
                    Ok(status) if status.success() => {
                        let flag = get_flag();
                        if let Ok(mut f) = flag.lock() {
                            *f = false;
                        }

                        let flag_clone = flag.clone();

                        thread::spawn(move || {
                            for _i in 0..timeout_secs {
                                thread::sleep(Duration::from_secs(1));
                                if let Ok(f) = flag_clone.lock() {
                                    if *f {
                                        return;
                                    }
                                }
                            }

                            let _ = Command::new("pkill").arg("wl-copy").status();

                            thread::sleep(Duration::from_millis(200));

                            let _ = Command::new("cliphist").arg("wipe").status();
                            let _ = Command::new("copyq").arg("clear").status();
                            let _ = Command::new("clipman").arg("clear").arg("--all").status();
                            let _ = Command::new("pkill").arg("wl-clip-persist").status();
                            let _ = Command::new("clipster").arg("-d").status();
                        });

                        let expires_at = crate::get_timestamp() + timeout_secs;
                        return ClipboardResult {
                            success: true,
                            message: format!("Copied! Clears in {timeout_secs}s"),
                            expires_at,
                        };
                    }
                    Ok(status) => {
                        return ClipboardResult {
                            success: false,
                            message: format!("wl-copy failed: {status}"),
                            expires_at: 0,
                        };
                    }
                    Err(e) => {
                        return ClipboardResult {
                            success: false,
                            message: format!("Process error: {e}"),
                            expires_at: 0,
                        };
                    }
                }
            }
        }
        Err(_e) => {
            return ClipboardResult {
                success: false,
                message: "wl-copy not found".to_string(),
                expires_at: 0,
            };
        }
    }

    ClipboardResult {
        success: false,
        message: "Unknown error".to_string(),
        expires_at: 0,
    }
}

#[allow(dead_code)]
pub fn clear_clipboard() {
    let _ = Command::new("pkill").arg("wl-copy").status();
}
