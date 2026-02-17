use arboard::Clipboard;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Global clipboard clear handle
static CLEAR_FLAG: std::sync::OnceLock<Arc<Mutex<bool>>> = std::sync::OnceLock::new();

fn get_flag() -> Arc<Mutex<bool>> {
    CLEAR_FLAG.get_or_init(|| Arc::new(Mutex::new(false))).clone()
}

pub struct ClipboardResult {
    pub success: bool,
    pub message: String,
    pub expires_at: u64,
}

pub fn copy_with_timeout(text: &str, timeout_secs: u64) -> ClipboardResult {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.set_text(text.to_string()) {
                Ok(()) => {
                    if let Ok(mut flag) = get_flag().lock() {
                        *flag = true;
                    }

                    let flag = get_flag();
                    if let Ok(mut f) = flag.lock() {
                        *f = false;
                    }

                    let flag_clone = flag.clone();
                    let timeout = timeout_secs;

                    thread::spawn(move || {
                        let sleep_interval = Duration::from_millis(500);
                        let steps = (timeout * 1000) / 500;

                        for _ in 0..steps {
                            thread::sleep(sleep_interval);
                            if let Ok(f) = flag_clone.lock() {
                                if *f {
                                    return;
                                }
                            }
                        }

                        if let Ok(mut cb) = Clipboard::new() {
                            let _ = cb.set_text(String::new());
                        }
                    });

                    let expires_at = crate::get_timestamp() + timeout_secs;
                    ClipboardResult {
                        success: true,
                        message: format!("Copied! Clears in {timeout_secs}s"),
                        expires_at,
                    }
                }
                Err(e) => ClipboardResult {
                    success: false,
                    message: format!("Clipboard error: {e}"),
                    expires_at: 0,
                },
            }
        }
        Err(e) => ClipboardResult {
            success: false,
            message: format!("Cannot access clipboard: {e}"),
            expires_at: 0,
        },
    }
}

pub fn clear_clipboard() {
    if let Ok(mut clipboard) = Clipboard::new() {
        let _ = clipboard.set_text(String::new());
    }
}

pub fn get_countdown(expires_at: u64) -> Option<u64> {
    let now = crate::get_timestamp();
    if expires_at > now {
        Some(expires_at - now)
    } else {
        None
    }
}