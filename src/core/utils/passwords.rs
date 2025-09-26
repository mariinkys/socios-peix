#[cfg(feature = "ssr")]
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

#[cfg(feature = "ssr")]
pub fn encrypt_password(password: String) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(password_hash) => Ok(password_hash.to_string()),
        Err(e) => Err(format!("Failed to hash password: {}", e)),
    }
}

#[cfg(feature = "ssr")]
pub fn verify_password(password: String, hash: String) -> Result<(), String> {
    let parsed_hash = match PasswordHash::new(&hash) {
        Ok(hash) => hash,
        Err(e) => return Err(format!("Invalid hash format: {}", e)),
    };

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Passwords do not match: {}", err)),
    }
}

// cargo test --features ssr -- --nocapture
#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[cfg(feature = "ssr")]
    #[test]
    fn test_encrypt_password() {
        let password = "admin".to_string();
        let result = encrypt_password(password);

        assert!(result.is_ok());
        let hash = result.unwrap();

        // Print the generated hash
        println!("Generated hash: {}", hash);

        // Verify the hash is not empty and has the expected Argon2 format
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }
}
