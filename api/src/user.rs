use axum::http::StatusCode;
use axum::{routing::post, Extension, Json, Router};
use once_cell::sync::Lazy;
use regex::Regex;

use db::SquirePool;
use serde::Deserialize;
use validator::Validate;

use crate::password::Password;
use crate::{Error, Result};

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
        .map_err(|e| {
            if e.constraint_error() {
                Error::Conflict("email taken".into())
            } else {
                e.into()
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn should_create_user() {
        let client = TestClient::new(router());
        let res = client
            .post("/v1/user")
            .body(r#"{"email":"should_create_user@test.com","password":"pwd"}"#)
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
