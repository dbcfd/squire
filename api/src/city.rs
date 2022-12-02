use axum::http::StatusCode;
use axum::{Extension, Json, Router};

use axum::routing::get;
use axum_auth::AuthBasic;

use db::{DateTime, SquirePool};
use serde::{Deserialize, Serialize};

use crate::auth::UserAuth;
use crate::{Error, Result};
use validator::Validate;

use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/v1/favorite/city", get(get_cities).post(create_city))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct CreateCity {
    #[validate(length(min = 1, max = 1000))]
    city: String,
    #[validate(length(min = 1, max = 1000))]
    country: String,
}

async fn create_city(
    db: Extension<SquirePool>,
    AuthBasic((id, password)): AuthBasic,
    Json(req): Json<CreateCity>,
) -> Result<StatusCode> {
    req.validate()?;
    let password = password.ok_or_else(|| Error::Auth)?;
    let user = UserAuth {
        email: id,
        password: password,
    }
    .verify(&*db)
    .await?;

    db::City::insert(&db, &user.id, &req.city, &req.country).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Representation of a city
#[serde_with::serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    id: Uuid,
    city: String,
    country: String,
    #[serde_as(as = "Rfc3339")]
    created_at: DateTime,
    #[serde_as(as = "Rfc3339")]
    updated_at: DateTime,
}

/// There's probably a way to just use the db City type, but sqlx is being annoying
impl From<db::City> for City {
    fn from(v: db::City) -> Self {
        City {
            id: v.id,
            city: v.city,
            country: v.country,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

/// Returns posts in descending chronological order.
async fn get_cities(
    db: Extension<SquirePool>,
    AuthBasic((id, password)): AuthBasic,
) -> Result<Json<Vec<City>>> {
    let password = password.ok_or_else(|| Error::Auth)?;
    let user = UserAuth {
        email: id,
        password: password,
    }
    .verify(&*db)
    .await?;

    // Note: normally you'd want to put a `LIMIT` on this as well,
    // though that would also necessitate implementing pagination.
    let cities = db::City::get(&db, &user.id).await?;
    let cities: Vec<_> = cities.into_iter().map(City::from).collect();

    Ok(Json(cities))
}
