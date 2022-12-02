use anyhow::anyhow;

use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub struct Password(String);

impl Password {
    pub fn hash(&self) -> anyhow::Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(self.0.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("failed to hash password"))?
            .to_string())
    }

    pub fn verify(&self, user: &db::User) -> anyhow::Result<bool> {
        let hash = PasswordHash::new(&user.hashed_password)
            .map_err(|e| anyhow!(e).context("BUG: password hash invalid"))?;

        let res = Argon2::default().verify_password(self.0.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("failed to verify password")),
        }
    }
}

impl From<String> for Password {
    fn from(v: String) -> Self {
        Self(v)
    }
}
