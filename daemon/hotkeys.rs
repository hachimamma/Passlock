use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    Capture,   // Ctrl+Shift+P - Capture password
    AutoFill,  // Ctrl+Shift+A - Auto-fill
    Lock,      // Ctrl+Shift+L - Lock vault
}

pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    capture_hotkey: HotKey,
    autofill_hotkey: HotKey,
    lock_hotkey: HotKey,
}

impl HotkeyManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()?;

        // Ctrl+Shift+P - Capture
        let capture_hotkey = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::SHIFT),
            Code::KeyP,
        );

        // Ctrl+Shift+A - Auto-fill
        let autofill_hotkey = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::SHIFT),
            Code::KeyA,
        );

        // Ctrl+Shift+L - Lock
        let lock_hotkey = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::SHIFT),
            Code::KeyL,
        );

        manager.register(capture_hotkey)?;
        manager.register(autofill_hotkey)?;
        manager.register(lock_hotkey)?;

        println!("Hotkeys registered:");
        println!("   Ctrl+Shift+P - Capture password");
        println!("   Ctrl+Shift+A - Auto-fill");
        println!("   Ctrl+Shift+L - Lock vault");

        Ok(Self {
            manager,
            capture_hotkey,
            autofill_hotkey,
            lock_hotkey,
        })
    }

    pub fn create_event_channel() -> (mpsc::Sender<HotkeyAction>, mpsc::Receiver<HotkeyAction>) {
        mpsc::channel(32)
    }

    pub async fn listen(&self, tx: mpsc::Sender<HotkeyAction>) -> Result<(), Box<dyn std::error::Error>> {
        let receiver = GlobalHotKeyEvent::receiver();

        loop {
            if let Ok(event) = receiver.try_recv() {
                let action = if event.id == self.capture_hotkey.id() {
                    Some(HotkeyAction::Capture)
                } else if event.id == self.autofill_hotkey.id() {
                    Some(HotkeyAction::AutoFill)
                } else if event.id == self.lock_hotkey.id() {
                    Some(HotkeyAction::Lock)
                } else {
                    None
                };

                if let Some(action) = action {
                    tx.send(action).await?;
                }
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        let _ = self.manager.unregister(self.capture_hotkey);
        let _ = self.manager.unregister(self.autofill_hotkey);
        let _ = self.manager.unregister(self.lock_hotkey);
    }
}