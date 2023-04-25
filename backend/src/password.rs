use thiserror::Error;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

#[derive(Error, Debug)]
#[error("Error hashing password")]
pub struct PasswordHashError;

#[derive(Error, Debug)]
#[error("Error comparing passwords")]
pub struct PasswordCompareError;

pub fn hash_password(password: String) -> Result<String, PasswordHashError> {
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| PasswordHashError)?
        .to_string();

    Ok(password_hash)
}

pub fn compare_password(
    password: String,
    password_hash: String,
) -> Result<bool, PasswordCompareError> {
    let argon2 = Argon2::default();

    let pased_password_hash =
        PasswordHash::new(&password_hash).map_err(|_| PasswordCompareError)?;

    let do_passwords_match = argon2
        .verify_password(password.as_bytes(), &pased_password_hash)
        .is_ok();

    Ok(do_passwords_match)
}
