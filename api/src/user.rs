use axum::http::StatusCode;
use axum::{routing::post, Extension, Json, Router};
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use std::time::Duration;

use db::SquirePool;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::http::{Error, Result};
use crate::password::Password;

pub type UserId = Uuid;

pub fn router() -> Router {
    Router::new().route("/v1/user", post(create_user))
}

// Just a simple email regex, could be improved, but good enough for this use case
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/^\S+@\S+\.\S+$/").unwrap());

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    #[validate(length(min = 3, max = 16), regex = "EMAIL_REGEX")]
    email: String,
    #[validate(length(min = 2, max = 64))]
    name: String,
    #[validate(length(min = 8, max = 32))]
    password: String,
}

// WARNING: this API has none of the checks that a normal user signup flow implements,
// such as email or phone verification.
async fn create_user(db: Extension<PgPool>, Json(req): Json<UserAuth>) -> Result<StatusCode> {
    req.validate()?;

    let UserAuth {
        email,
        name,
        password,
    } = req;

    let hashed_password = Password::from(password).hash()?;

    db::User::insert(db, email, name, hashed_password)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
                Error::Conflict("username taken".into())
            }
            _ => e.into(),
        })?;

    Ok(StatusCode::NO_CONTENT)
}

impl UserAuth {
    pub async fn verify(self, db: impl PgExecutor<'_> + Send) -> Result<User> {
        self.validate()?;

        if let Some(user) = db::User::get(db, &self.email).await? {
            if Password::from(self.password).verify(&user) {
                return Ok(user);
            }
        } else {
            // Sleep a random amount of time to avoid leaking existence of a user in timing.
            let sleep_duration = rand::thread_rng()
                .gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
            tokio::time::sleep(sleep_duration).await;
        }

        Err(Error::UnprocessableEntity(
            "invalid username/password".into(),
        ))
    }
}
