use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar};
use std::ptr;

const VAULT_SUCCESS: c_int = 0;
const VAULT_ERROR_AUTH: c_int = -4;
pub const SALT_LENGTH: usize = 16;

// Cipher type consts
pub const _CIPHER_AUTO: c_int = 0;
pub const _CIPHER_AES256GCM: c_int = 1;
pub const _CIPHER_CHACHA20POLY1305: c_int = 2;

#[link(name = "vault_engine", kind = "static")]
unsafe extern "C" {
    fn vault_init() -> c_int;
    fn vault_cleanup();

    fn vault_encrypt(
        pltext: *const c_uchar,
        pltext_len: usize,
        pwd: *const c_char,
        pwd_len: usize,
        salt: *const c_uchar,
        citext_out: *mut *mut c_uchar,
        citext_len_out: *mut usize,
    ) -> c_int;

    fn vault_decrypt(
        citext: *const c_uchar,
        citext_len: usize,
        pwd: *const c_char,
        pwd_len: usize,
        salt: *const c_uchar,
        pltext_out: *mut *mut c_uchar,
        pltext_len_out: *mut usize,
    ) -> c_int;

    fn vault_gen_salt(salt: *mut c_uchar, salt_len: usize) -> c_int;

    fn vault_free_buffer(buf: *mut c_uchar);

    fn vault_secure_zero(ptr: *mut c_uchar, len: usize);

    fn vault_aes_ni() -> c_int;

    #[allow(dead_code)]
    fn vault_cipher(
        pltext: *const c_uchar,
        pltext_len: usize,
        pwd: *const c_char,
        pwd_len: usize,
        salt: *const c_uchar,
        citext_out: *mut *mut c_uchar,
        citext_len_out: *mut usize,
        ci_type: c_int,
    ) -> c_int;
}

pub fn init() -> Result<(), String> {
    unsafe {
        if vault_init() == VAULT_SUCCESS {
            Ok(())
        } else {
            Err("Failed to initialize vault engine".to_string())
        }
    }
}

pub fn cleanup() {
    unsafe {
        vault_cleanup();
    }
}

pub fn generate_salt() -> Result<Vec<u8>, String> {
    let mut salt = vec![0u8; SALT_LENGTH];
    unsafe {
        if vault_gen_salt(salt.as_mut_ptr(), SALT_LENGTH) == VAULT_SUCCESS {
            Ok(salt)
        } else {
            Err("Failed to generate salt".to_string())
        }
    }
}

pub fn aes_sup() -> bool {
    unsafe { vault_aes_ni() != 0 }
}

pub fn get_cipher() -> &'static str {
    if aes_sup() {
        "AES-256-GCM (hardware accelerated)"
    } else {
        "ChaCha20-Poly1305 (optimized for your CPU)"
    }
}

#[allow(dead_code)]
pub fn get_name(ci_type: c_int) -> &'static str {
    match ci_type {
        _CIPHER_AES256GCM => "AES-256-GCM",
        _CIPHER_CHACHA20POLY1305 => "ChaCha20-Poly1305",
        _ => "Auto",
    }
}

pub fn encrypt_data(pltext: &[u8], pwd: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    if salt.len() != SALT_LENGTH {
        return Err(format!(
            "Invalid salt length: expected {}, got {}",
            SALT_LENGTH,
            salt.len()
        ));
    }

    let pwd_cstr = CString::new(pwd).map_err(|_| "Invalid pwd string")?;

    let mut citext_ptr: *mut c_uchar = ptr::null_mut();
    let mut citext_len: usize = 0;

    unsafe {
        let result = vault_encrypt(
            pltext.as_ptr(),
            pltext.len(),
            pwd_cstr.as_ptr(),
            pwd.len(),
            salt.as_ptr(),
            &raw mut citext_ptr,
            &raw mut citext_len,
        );

        if result == VAULT_SUCCESS {
            let citext = std::slice::from_raw_parts(citext_ptr, citext_len).to_vec();
            vault_free_buffer(citext_ptr);
            Ok(citext)
        } else {
            if !citext_ptr.is_null() {
                vault_free_buffer(citext_ptr);
            }
            Err("Encryption failed".to_string())
        }
    }
}

#[allow(dead_code)]
pub fn encrypt_data_with_cipher(
    pltext: &[u8],
    pwd: &str,
    salt: &[u8],
    ci_type: c_int,
) -> Result<Vec<u8>, String> {
    if salt.len() != SALT_LENGTH {
        return Err(format!(
            "Invalid salt length: expected {}, got {}",
            SALT_LENGTH,
            salt.len()
        ));
    }

    let pwd_cstr = CString::new(pwd).map_err(|_| "Invalid pwd string")?;

    let mut citext_ptr: *mut c_uchar = ptr::null_mut();
    let mut citext_len: usize = 0;

    unsafe {
        let result = vault_cipher(
            pltext.as_ptr(),
            pltext.len(),
            pwd_cstr.as_ptr(),
            pwd.len(),
            salt.as_ptr(),
            &raw mut citext_ptr,
            &raw mut citext_len,
            ci_type,
        );

        if result == VAULT_SUCCESS {
            let citext = std::slice::from_raw_parts(citext_ptr, citext_len).to_vec();
            vault_free_buffer(citext_ptr);
            Ok(citext)
        } else {
            if !citext_ptr.is_null() {
                vault_free_buffer(citext_ptr);
            }
            Err("Encryption failed".to_string())
        }
    }
}

pub fn decrypt_data(citext: &[u8], pwd: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    if salt.len() != SALT_LENGTH {
        return Err(format!(
            "Invalid salt length: expected {}, got {}",
            SALT_LENGTH,
            salt.len()
        ));
    }

    let pwd_cstr = CString::new(pwd).map_err(|_| "Invalid pwd string")?;

    let mut plaintext_ptr: *mut c_uchar = ptr::null_mut();
    let mut pltext_len: usize = 0;

    unsafe {
        let result = vault_decrypt(
            citext.as_ptr(),
            citext.len(),
            pwd_cstr.as_ptr(),
            pwd.len(),
            salt.as_ptr(),
            &raw mut plaintext_ptr,
            &raw mut pltext_len,
        );

        if result == VAULT_SUCCESS {
            let pltext = std::slice::from_raw_parts(plaintext_ptr, pltext_len).to_vec();
            vault_free_buffer(plaintext_ptr);
            Ok(pltext)
        } else {
            if !plaintext_ptr.is_null() {
                vault_free_buffer(plaintext_ptr);
            }
            if result == VAULT_ERROR_AUTH {
                Err("Wrong pwd".to_string())
            } else {
                Err("Decryption failed".to_string())
            }
        }
    }
}

pub fn secure_zero(data: &mut [u8]) {
    unsafe {
        vault_secure_zero(data.as_mut_ptr(), data.len());
    }
}
