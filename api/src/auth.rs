use crate::{password::Password, Error, Result};
use db::{SquirePool, User};
use rand::Rng;
use std::time::Duration;

pub struct UserAuth {
    pub email: String,
    pub password: String,
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

        Err(Error::UnprocessableEntity("invalid email/password".into()))
    }
}
