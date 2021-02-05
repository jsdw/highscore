/// Hash a password string.
pub fn hash(password: &str) -> String {
    let config = argon2::Config::default();
    let mut salt = [0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut salt);
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

/// Verify that a password equals a hashed version
pub fn verify(hashed_password: &str, password: &str) -> bool {
    argon2::verify_encoded(&hashed_password, password.as_bytes()).unwrap_or(false)
}