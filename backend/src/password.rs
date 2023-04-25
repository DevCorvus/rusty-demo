use anyhow;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_password(password: String) -> anyhow::Result<String> {
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(anyhow::Error::msg)?
        .to_string();

    Ok(password_hash)
}

pub fn compare_password(password: String, password_hash: String) -> anyhow::Result<bool> {
    let argon2 = Argon2::default();

    let pased_password_hash = PasswordHash::new(&password_hash).map_err(anyhow::Error::msg)?;

    let do_passwords_match = argon2
        .verify_password(password.as_bytes(), &pased_password_hash)
        .is_ok();

    Ok(do_passwords_match)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() -> anyhow::Result<()> {
        let password = String::from("123456");
        let password_hash = hash_password(password.clone())?;
        assert_ne!(password, password_hash);

        Ok(())
    }

    #[test]
    fn test_compare_password() -> anyhow::Result<()> {
        let password = String::from("123456");
        let password_hash = hash_password(password.clone())?;
        assert!(compare_password(password, password_hash)?);

        Ok(())
    }
}
