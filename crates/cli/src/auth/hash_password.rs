use argon2::{
    Argon2, 
    PasswordHash, 
    PasswordHasher, 
    PasswordVerifier, 
    password_hash::{SaltString, rand_core::OsRng}
};

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash_password = argon2.hash_password(&password.as_bytes(), &salt).unwrap().to_string();
    
    hash_password
}

fn verify_password(password: String, hash_password: String) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let parsed_password = PasswordHash::new(&hash_password).unwrap();
    
    argon2.verify_password(password.as_bytes(), &parsed_password)
}