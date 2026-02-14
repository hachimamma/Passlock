#ifndef VAULT_ENGINE_H
#define VAULT_ENGINE_H

#include <stddef.h>
#include <stdint.h>

// Return codes
#define VAULT_SUCCESS 0
#define VAULT_ERROR -1
#define VAULT_ERROR_MEMORY -2
#define VAULT_ERROR_CRYPTO -3
#define VAULT_ERROR_AUTH -4

// Cryptographic constants
#define SALT_LENGTH 16
#define KEY_LENGTH 32
#define NONCE_LENGTH 12
#define TAG_LENGTH 16

typedef struct {
    unsigned char *data;
    size_t length;
} VaultBuffer;

// Initialization and cleanup
int vault_init(void);
void vault_cleanup(void);

// Core encryption/decryption
int vault_encrypt(
    const unsigned char *plaintext,
    size_t plaintext_len,
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char **ciphertext_out,
    size_t *ciphertext_len_out
);

int vault_decrypt(
    const unsigned char *ciphertext,
    size_t ciphertext_len,
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char **plaintext_out,
    size_t *plaintext_len_out
);

// Encryption with cipher selection
int vault_encrypt_with_cipher(
    const unsigned char *plaintext,
    size_t plaintext_len,
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char **ciphertext_out,
    size_t *ciphertext_len_out,
    int cipher_type
);

// Legacy decryption (backward compatibility)
int vault_decrypt_legacy(
    const unsigned char *ciphertext,
    size_t ciphertext_len,
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char **plaintext_out,
    size_t *plaintext_len_out
);

// Key derivation and salt gen
int vault_derive_key(
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char *key_out
);

int vault_gen_salt(unsigned char *salt, size_t salt_len);

// Memory management
void vault_free_buffer(unsigned char *buf);
void vault_secure_zero(void *ptr, size_t len);

/**
 * @brief Safe memory copy function
 * 
 * @param dest Destination buffer
 * @param src Source buffer
 * @param n Number of bytes to copy
 * @return void* Pointer to destination buffer, or NULL on error
 * 
 * This function performs basic safety checks before calling memcpy.
 * It prevents null pointer dereferences and zero-length copies.
 */
void *vault_memcpy(void *dest, const void *src, size_t n);

// CPU feature detection
/**
 * @brief Detect if CPU has AES-NI hardware acceleration
 * 
 * @return int 1 if AES-NI is available, 0 otherwise
 * 
 * On x86/x64 systems, this checks CPUID for AES-NI support.
 * On other architectures (ARM, RISC-V), returns 0.
 * 
 * AES-NI accelerates AES-256-GCM by 10-20x. Without it,
 * ChaCha20-Poly1305 is typically 5-6x faster than software AES.
 */
int vault_aes_ni(void);

#endif