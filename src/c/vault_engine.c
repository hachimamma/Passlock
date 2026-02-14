#include "vault_engine.h"
#include <sodium.h>
#include <string.h>
#include <stdlib.h>

//  CPU feature detect
#if defined(__x86_64__) || defined(_M_X64) || defined(__i386__) || defined(_M_IX86)
    #include <cpuid.h>
    #define VAULT_X86 1
#endif

typedef enum {
    CIPHER_AUTO = 0,
    CIPHER_AES256GCM = 1,
    CIPHER_CHACHA20POLY1305 = 2
} vault_cipher_t;

__attribute__((used))
int vault_aes_ni(void) {
#ifdef VAULT_X86
    unsigned int eax, ebx, ecx, edx;
    
    // check for CPUID
    if (__get_cpuid(1, &eax, &ebx, &ecx, &edx)) {
        // AES-NI is bit 25 of ECX
        return (ecx & (1 << 25)) != 0;
    }
#endif
    return 0; // No AES-NI support
}

// auto select
__attribute__((used))
vault_cipher_t vault_auto_select_cipher(void) {
    // cppcheck-suppress knownConditionTrueFalse
    if (vault_aes_ni()) {
        return CIPHER_AES256GCM;
    } else {
        return CIPHER_CHACHA20POLY1305;
    }
}

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
    
    unsigned char *d = (unsigned char *)dest;
    const unsigned char *s = (const unsigned char *)src;
    
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    
    return dest;
}

__attribute__((used))
int vault_encrypt_with_cipher(
    const unsigned char *plaintext,
    size_t plaintext_len,
    const char *password,
    size_t password_len,
    const unsigned char *salt,
    unsigned char **ciphertext_out,
    size_t *ciphertext_len_out,
    int cipher
) {
    if (!plaintext || !password || !salt || !ciphertext_out || !ciphertext_len_out) {
        return VAULT_ERROR;
    }

    vault_cipher_t cipher_type = (vault_cipher_t)cipher;

    if (cipher_type == CIPHER_AUTO) {
        cipher_type = vault_auto_select_cipher();
    }

    unsigned char key[KEY_LENGTH];
    unsigned char nonce[NONCE_LENGTH];
    
    if (vault_derive_key(password, password_len, salt, key) != VAULT_SUCCESS) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_CRYPTO;
    }

    randombytes_buf(nonce, NONCE_LENGTH);

    size_t ciphertext_len = 1 + NONCE_LENGTH + plaintext_len + TAG_LENGTH;
    unsigned char *ciphertext = malloc(ciphertext_len);
    if (!ciphertext) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_MEMORY;
    }

    ciphertext[0] = (unsigned char)cipher_type;

    vault_memcpy(ciphertext + 1, nonce, NONCE_LENGTH);

    unsigned long long actual_ciphertext_len;
    int encrypt_result;

    if (cipher_type == CIPHER_AES256GCM) {
        // Use AES-256-GCM (fast for AES-NI CPUs)
        encrypt_result = crypto_aead_aes256gcm_encrypt(
            ciphertext + 1 + NONCE_LENGTH,
            &actual_ciphertext_len,
            plaintext,
            plaintext_len,
            NULL,
            0,
            NULL,
            nonce,
            key
        );
    } else {
        // Use ChaCha20-Poly1305
        encrypt_result = crypto_aead_chacha20poly1305_ietf_encrypt(
            ciphertext + 1 + NONCE_LENGTH,
            &actual_ciphertext_len,
            plaintext,
            plaintext_len,
            NULL,
            0,
            NULL,
            nonce,
            key
        );
    }

    if (encrypt_result != 0) {
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

// Public encrypt func
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
    return vault_encrypt_with_cipher(
        plaintext,
        plaintext_len,
        password,
        password_len,
        salt,
        ciphertext_out,
        ciphertext_len_out,
        0  // CIPHER_AUTO = 0
    );
}

// Legacy decrypt function (before cipher versioning)
__attribute__((used))
int vault_decrypt_legacy(
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
    
    // Try ChaCha20 first (current default after merge)
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

    // Need at least: 1 byte (cipher) + nonce + tag
    if (ciphertext_len < 1 + NONCE_LENGTH + TAG_LENGTH) {
        // Legacy format detection: no cipher byte, assume ChaCha20
        // (for backward compatibility with just-merged PR from @rocky)
        if (ciphertext_len >= NONCE_LENGTH + TAG_LENGTH) {
            return vault_decrypt_legacy(
                ciphertext,
                ciphertext_len,
                password,
                password_len,
                salt,
                plaintext_out,
                plaintext_len_out
            );
        }
        return VAULT_ERROR;
    }

    vault_cipher_t cipher = (vault_cipher_t)ciphertext[0];
    
    if (cipher != CIPHER_AES256GCM && cipher != CIPHER_CHACHA20POLY1305) {
        // Might be legacy format - try ChaCha20
        return vault_decrypt_legacy(
            ciphertext,
            ciphertext_len,
            password,
            password_len,
            salt,
            plaintext_out,
            plaintext_len_out
        );
    }

    unsigned char key[KEY_LENGTH];
    
    if (vault_derive_key(password, password_len, salt, key) != VAULT_SUCCESS) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_CRYPTO;
    }

    const unsigned char *nonce = ciphertext + 1;
    const unsigned char *encrypted_data = ciphertext + 1 + NONCE_LENGTH;
    size_t encrypted_data_len = ciphertext_len - 1 - NONCE_LENGTH;

    size_t plaintext_len = encrypted_data_len - TAG_LENGTH;
    unsigned char *plaintext = malloc(plaintext_len);
    if (!plaintext) {
        vault_secure_zero(key, KEY_LENGTH);
        return VAULT_ERROR_MEMORY;
    }

    unsigned long long actual_plaintext_len;
    int decrypt_result;

    if (cipher == CIPHER_AES256GCM) {
        decrypt_result = crypto_aead_aes256gcm_decrypt(
            plaintext,
            &actual_plaintext_len,
            NULL,
            encrypted_data,
            encrypted_data_len,
            NULL,
            0,
            nonce,
            key
        );
    } else {
        decrypt_result = crypto_aead_chacha20poly1305_ietf_decrypt(
            plaintext,
            &actual_plaintext_len,
            NULL,
            encrypted_data,
            encrypted_data_len,
            NULL,
            0,
            nonce,
            key
        );
    }

    if (decrypt_result != 0) {
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