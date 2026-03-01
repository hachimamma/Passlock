use crate::crypto;
use crate::models::Vault;
use std::fs;
use std::path::PathBuf;

/// Get vault path for a specific vault register
fn vt_p_named(vault_name: &str) -> PathBuf {
    crate::config::get_vault_path(vault_name)
}

/// Save vault to a specific vault register
pub fn svv_named(vault_name: &str, v: &Vault, pwd: &str) -> Result<(), String> {
    let vault_path = vt_p_named(vault_name);

    if let Some(parent) = vault_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let j = serde_json::to_string(v).map_err(|e| e.to_string())?;
    let j_bytes = j.as_bytes();
    let enc_d = crypto::enc(j_bytes, pwd, &v.s)?;
    let salt_bytes = hex::decode(&v.s).map_err(|_| "Invalid salt")?;
    let mut __final_data__ = Vec::new();
    __final_data__.extend_from_slice(&salt_bytes);
    __final_data__.extend_from_slice(&enc_d);
    fs::write(vault_path, __final_data__).map_err(|e| e.to_string())?;

    let tmp_j = serde_json::to_string(v).map_err(|e| e.to_string())?;
    fs::write(tmp_p(), tmp_j).map_err(|e| e.to_string())?;

    Ok(())
}

/// Load vault from a specific vault register
pub fn ld_vt_named(vault_name: &str, pwd: &str) -> Result<Vault, String> {
    let vault_path = vt_p_named(vault_name);

    if !vault_path.exists() {
        return Err(format!("Vault '{vault_name}' not found"));
    }

    let data = fs::read(vault_path).map_err(|_| "vault not found")?;
    if data.len() < 16 {
        return Err("corrupt vault".to_string());
    }
    let salt_bytes = &data[0..16];
    let enc_data = &data[16..];
    let salt = hex::encode(salt_bytes);
    let dec_data = crypto::dec(enc_data, pwd, &salt)?;
    let dec_str = String::from_utf8(dec_data).map_err(|_| "invalid data")?;
    let v: Vault = serde_json::from_str(&dec_str).map_err(|e| e.to_string())?;

    let tmp_j = serde_json::to_string(&v).map_err(|e| e.to_string())?;
    fs::write(tmp_p(), tmp_j).map_err(|e| e.to_string())?;

    Ok(v)
}

/// Check for a specific vault
pub fn vt_exi_named(vault_name: &str) -> bool {
    vt_p_named(vault_name).exists()
}

/// Helper for vault creation
pub fn save_vault_to(vault_name: &str, v: &Vault, pwd: &str) -> Result<(), String> {
    svv_named(vault_name, v, pwd)
}

// backward compatible

/// Get vault path
fn _vt_p() -> PathBuf {
    let cfg = crate::config::load_config().unwrap_or_default();
    vt_p_named(&cfg.active_vault)
}

/// Get temp file path
fn tmp_p() -> PathBuf {
    let home = dirs::home_dir().expect("no home");
    home.join(".passlock.temp")
}

/// Save vault
pub fn svv(v: &Vault, pwd: &str) -> Result<(), String> {
    let cfg = crate::config::load_config().unwrap_or_default();
    svv_named(&cfg.active_vault, v, pwd)
}

/// Load vault
pub fn ld_vt(pwd: &str) -> Result<Vault, String> {
    let cfg = crate::config::load_config().unwrap_or_default();
    ld_vt_named(&cfg.active_vault, pwd)
}

/// Check if vault exists
pub fn vt_exi() -> bool {
    let cfg = crate::config::load_config().unwrap_or_default();
    vt_exi_named(&cfg.active_vault)
}
