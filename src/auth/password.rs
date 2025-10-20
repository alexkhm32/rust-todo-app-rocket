use crate::domain::{Error, PasswordHasher as PasswordHasherTrait};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher as ArgonPasswordHasher, SaltString, rand_core::OsRng},
};

pub struct PasswordHasher;

impl PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasherTrait for PasswordHasher {
    fn hash(&self, password: String) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let hash = argon
            .hash_password(password.as_bytes(), salt.as_salt())
            .unwrap()
            .to_string();
        Ok(hash)
    }

    fn verify(&self, password: String, actual: String) -> bool {
        let parsed = PasswordHash::new(&actual).unwrap();
        let argon = Argon2::default();
        argon.verify_password(password.as_bytes(), &parsed).is_ok()
    }
}
