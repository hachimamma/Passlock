use crate::daemon::core::PassLockDaemon;
use crate::models::Entry;
use std::sync::Arc;
use tokio::sync::Mutex;

// Simple REST API for daemon communication
// This would be expanded with actual HTTP server implementation

pub struct DaemonApi {
    daemon: Arc<Mutex<PassLockDaemon>>,
    port: u16,
}

impl DaemonApi {
    pub fn new(daemon: Arc<Mutex<PassLockDaemon>>, port: u16) -> Self {
        Self { daemon, port }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🌐 API server starting on port {}", self.port);
        
        // In a real implementation, this would use axum or actix-web
        // For now, this is a placeholder for future expansion
        
        println!("API server ready at http://localhost:{}", self.port);
        Ok(())
    }

    // Example API methods that would be exposed:
    
    pub async fn status(&self) -> ApiStatus {
        let daemon = self.daemon.lock().await;
        ApiStatus {
            running: true,
            unlocked: daemon.is_unlocked().await,
            port: self.port,
        }
    }

    pub async fn unlock(&self, password: String) -> Result<(), String> {
        let daemon = self.daemon.lock().await;
        daemon.unlock_vault(&password).await
            .map_err(|e| e.to_string())
    }

    pub async fn lock(&self) -> Result<(), String> {
        let daemon = self.daemon.lock().await;
        daemon.lock_vault().await
            .map_err(|e| e.to_string())
    }

    pub async fn save_entry(&self, entry: Entry) -> Result<(), String> {
        let daemon = self.daemon.lock().await;
        daemon.save_entry(entry).await
            .map_err(|e| e.to_string())
    }

    pub async fn search(&self, query: String) -> Vec<Entry> {
        let daemon = self.daemon.lock().await;
        daemon.search_entries(&query).await
    }
}

#[derive(Debug, Clone)]
pub struct ApiStatus {
    pub running: bool,
    pub unlocked: bool,
    pub port: u16,
}

// Future: Add actual HTTP routes
// POST /api/unlock
// POST /api/lock
// POST /api/entries
// GET  /api/entries/search?q=
// GET  /api/status