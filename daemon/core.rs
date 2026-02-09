use crate::models::{Entry, Vault};
use crate::storage;
use crate::crypto;
use notify_rust::Notification;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

pub struct PassLockDaemon {
    vault: Arc<Mutex<Option<Vault>>>,
    master_password: Arc<Mutex<Option<String>>>,
    last_activity: Arc<Mutex<Instant>>,
    lock_timeout: Duration,
}

impl PassLockDaemon {
    pub fn new() -> Self {
        Self {
            vault: Arc::new(Mutex::new(None)),
            master_password: Arc::new(Mutex::new(None)),
            last_activity: Arc::new(Mutex::new(Instant::now())),
            lock_timeout: Duration::from_secs(15 * 60), // 15 minutes
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("PassLock daemon starting...");
        
        if !storage::vt_exi() {
            self.notify("No vault found. Create one with: passlock create <password>")?;
            return Err("No vault found".into());
        }

        self.notify("PassLock daemon started")?;
        println!("Press Ctrl+Shift+P to capture password");
        println!("Press Ctrl+Shift+A to auto-fill");
        println!("Press Ctrl+Shift+L to lock vault");
        
        let self_clone = self.clone_arc();
        tokio::spawn(async move {
            self_clone.auto_lock_monitor().await;
        });

        Ok(())
    }

    pub async fn unlock_vault(&self, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        match storage::ld_vt(password) {
            Ok(vault) => {
                *self.vault.lock().await = Some(vault);
                *self.master_password.lock().await = Some(password.to_string());
                *self.last_activity.lock().await = Instant::now();
                self.notify("Vault unlocked")?;
                Ok(())
            }
            Err(e) => {
                self.notify("Wrong password")?;
                Err(e.into())
            }
        }
    }

    pub async fn lock_vault(&self) -> Result<(), Box<dyn std::error::Error>> {
        *self.vault.lock().await = None;
        *self.master_password.lock().await = None;
        self.notify("Vault locked")?;
        Ok(())
    }

    pub async fn is_unlocked(&self) -> bool {
        self.vault.lock().await.is_some()
    }

    pub async fn save_entry(&self, entry: Entry) -> Result<(), Box<dyn std::error::Error>> {
        let mut vault_guard = self.vault.lock().await;
        let master_pwd_guard = self.master_password.lock().await;

        if let (Some(vault), Some(password)) = (vault_guard.as_mut(), master_pwd_guard.as_ref()) {
            vault.e.push(entry.clone());
            storage::svv(vault, password)?;
            *self.last_activity.lock().await = Instant::now();
            self.notify(&format!("Saved: {}", entry.n))?;
            Ok(())
        } else {
            self.notify("Vault is locked")?;
            Err("Vault is locked".into())
        }
    }

    pub async fn search_entries(&self, query: &str) -> Vec<Entry> {
        if let Some(vault) = self.vault.lock().await.as_ref() {
            let query_lower = query.to_lowercase();
            vault.e.iter()
                .filter(|e| {
                    e.n.to_lowercase().contains(&query_lower) ||
                    e.u.to_lowercase().contains(&query_lower) ||
                    e.url.as_ref().map_or(false, |u| u.to_lowercase().contains(&query_lower))
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    async fn auto_lock_monitor(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            
            let last = *self.last_activity.lock().await;
            let is_unlocked = self.is_unlocked().await;
            
            if is_unlocked && last.elapsed() >= self.lock_timeout {
                let _ = self.lock_vault().await;
            }
        }
    }

    fn notify(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(not(target_os = "windows"))]
        {
            Notification::new()
                .summary("PassLock")
                .body(message)
                .timeout(3000)
                .show()?;
        }
        
        println!("{}", message);
        Ok(())
    }

    fn clone_arc(&self) -> Self {
        Self {
            vault: Arc::clone(&self.vault),
            master_password: Arc::clone(&self.master_password),
            last_activity: Arc::clone(&self.last_activity),
            lock_timeout: self.lock_timeout,
        }
    }
}

impl Default for PassLockDaemon {
    fn default() -> Self {
        Self::new()
    }
}