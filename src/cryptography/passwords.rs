use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt);
    let password_hash = password_hash.expect("should be able to hash password");
    password_hash.to_string()
}

pub fn verify_password(password: &[u8], phc_string: &str) -> bool {
    let parsed_hash = PasswordHash::new(phc_string)
        .expect("provided invalid phc string");
    Argon2::default().verify_password(password, &parsed_hash).is_ok()
}