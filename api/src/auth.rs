use db::{SquirePool, User};
use crate::{Error, password::Password, Result};
use std::time::Duration;
use rand::Rng;

pub struct UserAuth {
    email: String,
    password: String,
}

impl UserAuth {
    pub async fn verify(self, db: &SquirePool) -> Result<User> {
        if let Some(user) = db::User::get(db, &self.email).await? {
            if Password::from(self.password).verify(&user)? {
                return Ok(user);
            }
        } else {
            // Sleep a random amount of time to avoid leaking existence of a user in timing.
            let sleep_duration = rand::thread_rng()
                .gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
            tokio::time::sleep(sleep_duration).await;
        }

        Err(Error::UnprocessableEntity(
            "invalid email/password".into(),
        ))
    }
}