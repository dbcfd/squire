use axum::http::StatusCode;
use axum::{routing::post, Extension, Json, Router};
use once_cell::sync::Lazy;
use regex::Regex;

use db::SquirePool;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{Error, Result};
use crate::password::Password;

pub type UserId = Uuid;

pub fn router() -> Router {
    Router::new().route("/v1/user", post(create_user))
}

// Just a simple email regex, could be improved, but good enough for this use case
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/^\S+@\S+\.\S+$/").unwrap());

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserCreate {
    #[validate(length(min = 3, max = 16), regex = "EMAIL_REGEX")]
    email: String,
    #[validate(length(min = 2, max = 64))]
    name: String,
    #[validate(length(min = 8, max = 32))]
    password: String,
}

// WARNING: this API has none of the checks that a normal user signup flow implements,
// such as email or phone verification.
async fn create_user(db: Extension<SquirePool>, Json(req): Json<UserCreate>) -> Result<StatusCode> {
    req.validate()?;

    let UserCreate {
        email,
        name,
        password,
    } = req;

    let hashed_password = Password::from(password).hash()?;

    db::User::insert(&db, &email, &name, &hashed_password)
        .await
        .map_err(|e| match e {
            db::Error(sqlx::Error::Database(dbe)) if dbe.constraint() == Some("user_email_key") => {
                Error::Conflict("email taken".into())
            }
            _ => e.into(),
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn shuold_create_user() {
        let client = TestClient::new(router());
        let res = client.get("/v1/user").post().await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}