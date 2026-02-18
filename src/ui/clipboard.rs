use std::io::Write;
use std::process::{Command, Stdio};
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
                        let verify = Command::new("wl-paste").output();
                        if let Ok(output) = verify {
                            let _pasted = String::from_utf8_lossy(&output.stdout);
                        }

                        let flag = get_flag();
                        if let Ok(mut f) = flag.lock() {
                            *f = false;
                        }

                        let flag_clone = flag.clone();

                        thread::spawn(move || {
                            let sleep_interval = Duration::from_millis(500);
                            let steps = (timeout_secs * 1000) / 500;

                            for _ in 0..steps {
                                thread::sleep(sleep_interval);
                                if let Ok(f) = flag_clone.lock() {
                                    if *f {
                                        return;
                                    }
                                }
                            }

                            let _ = Command::new("wl-copy").arg("--clear").output();
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
                            message: format!("wl-copy exit: {status}"),
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
    let _ = Command::new("wl-copy").arg("--clear").output();
}