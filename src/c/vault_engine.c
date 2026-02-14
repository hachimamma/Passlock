#include "vault_engine.h"
#include <sodium.h>
#include <string.h>
#include <stdlib.h>

__attribute__((used))
int vault_init(void) {
    if (sodium_init() < 0) {
        return VAULT_ERROR;
    }
    return VAULT_SUCCESS;
}

__attribute__((used))
void vault_cleanup(void) {
}

__attribute__((used))
void vault_secure_zero(void *ptr, size_t len) {
    sodium_memzero(ptr, len);
}

__attribute__((used))
int vault_gen_salt(unsigned char *salt, size_t salt_len) {
    if (!salt || salt_len == 0) {
        return VAULT_ERROR;
    }
    randombytes_buf(salt, salt_len);
    return VAULT_SUCCESS;
}

__attribute__((used))
int vault_derive_key(
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char *key_out
) {
    if (!password || !salt || !key_out || password_len == 0) {
        return VAULT_ERROR;
    }

    if (crypto_pwhash(
            key_out,
            KEY_LENGTH,
            password,
            password_len,
            salt,
            crypto_pwhash_OPSLIMIT_INTERACTIVE,
            crypto_pwhash_MEMLIMIT_INTERACTIVE,
            crypto_pwhash_ALG_ARGON2ID13
        ) != 0) {
        return VAULT_ERROR_CRYPTO;
    }

    return VAULT_SUCCESS;
}

__attribute__((used))
void *vault_memcpy(void *dest, const void *src, size_t n) {
    if (dest == NULL || src == NULL || n == 0) {
        return dest;
    }
    
    // updated for byte check
    unsigned char *d = (unsigned char *)dest;
    const unsigned char *s = (const unsigned char *)src;
    
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    
    return dest;
}

__attribute__((used))
int vault_encrypt(
    const unsigned char *plaintext,
    size_t plaintext_len,
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char **ciphertext_out,
    size_t *ciphertext_len_out
) {
    if (!plaintext || !password || !salt || !ciphertext_out || !ciphertext_len_out) {
        return VAULT_ERROR;
    }

    unsigned char key[KEY_LENGTH];
    unsigned char nonce[NONCE_LENGTH];
    
    if (vault_derive_key(password, password_len, salt, key) != VAULT_SUCCESS) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_CRYPTO;
    }

    randombytes_buf(nonce, NONCE_LENGTH);

    size_t ciphertext_len = NONCE_LENGTH + plaintext_len + TAG_LENGTH;
    unsigned char *ciphertext = malloc(ciphertext_len);
    if (!ciphertext) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_MEMORY;
    }

    // safe mem copy with bounds checks
    if (ciphertext_len >= NONCE_LENGTH) {
        vault_memcpy(ciphertext, nonce, NONCE_LENGTH);
    } else {
        // shouldnt happen, but sanity and safety
        free(ciphertext);
        vault_secure_zero(key, KEY_LENGTH);
        vault_secure_zero(nonce, NONCE_LENGTH);
        return VAULT_ERROR;
    }

    unsigned long long actual_ciphertext_len;
    
    // switch from 'AES-256-GCM' to 'ChaCha20-Poly1305' for encrypt func
    if (crypto_aead_chacha20poly1305_ietf_encrypt(
            ciphertext + NONCE_LENGTH,
            &actual_ciphertext_len,
            plaintext,
            plaintext_len,
            NULL,
            0,
            NULL,
            nonce,
            key
        ) != 0) {
        free(ciphertext);
        vault_secure_zero(key, KEY_LENGTH);
        vault_secure_zero(nonce, NONCE_LENGTH);
        return VAULT_ERROR_CRYPTO;
    }

    vault_secure_zero(key, KEY_LENGTH);
    vault_secure_zero(nonce, NONCE_LENGTH);

    *ciphertext_out = ciphertext;
    *ciphertext_len_out = ciphertext_len;

    return VAULT_SUCCESS;
}

__attribute__((used))
int vault_decrypt(
    const unsigned char *ciphertext,
    size_t ciphertext_len,
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char **plaintext_out,
    size_t *plaintext_len_out
) {
    if (!ciphertext || !password || !salt || !plaintext_out || !plaintext_len_out) {
        return VAULT_ERROR;
    }

    if (ciphertext_len < NONCE_LENGTH + TAG_LENGTH) {
        return VAULT_ERROR;
    }

    unsigned char key[KEY_LENGTH];
    
    if (vault_derive_key(password, password_len, salt, key) != VAULT_SUCCESS) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_CRYPTO;
    }

    const unsigned char *nonce = ciphertext;
    const unsigned char *encrypted_data = ciphertext + NONCE_LENGTH;
    size_t encrypted_data_len = ciphertext_len - NONCE_LENGTH;

    size_t plaintext_len = encrypted_data_len - TAG_LENGTH;
    unsigned char *plaintext = malloc(plaintext_len);
    if (!plaintext) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_MEMORY;
    }

    unsigned long long actual_plaintext_len;
    
    // from 'AES-256-GCM' to 'ChaCha20-Poly1305' for decrypt func
    if (crypto_aead_chacha20poly1305_ietf_decrypt(
            plaintext,
            &actual_plaintext_len,
            NULL,
            encrypted_data,
            encrypted_data_len,
            NULL,
            0,
            nonce,
            key
        ) != 0) {
        free(plaintext);
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_AUTH;
    }

    vault_secure_zero(key, KEY_LENGTH);

    *plaintext_out = plaintext;
    *plaintext_len_out = actual_plaintext_len;

    return VAULT_SUCCESS;
}

__attribute__((used))
void vault_free_buffer(unsigned char *buf) {
    if (buf) {
        free(buf);
    }
}
