use argon2::{
    Argon2, 
    PasswordHash, 
    PasswordHasher, 
    PasswordVerifier, 
    password_hash::{SaltString, rand_core::OsRng}
};

pub fn hash_password(
    password: &str,
) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password(
    password: &str,
    hash_password: &str,
) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    let parsed_password = PasswordHash::new(hash_password)?;

    argon2.verify_password(password.as_bytes(), &parsed_password)
}