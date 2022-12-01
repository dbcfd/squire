use axum::{Extension, Json, Router};

use axum::routing::get;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::user::UserAuth;
use crate::Error;
use sqlx::PgPool;
use validator::Validate;

use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/v1/favorite/city", get(get_cities).post(create_city))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct CreateCity {
    auth: UserAuth,
    #[validate(length(min = 1, max = 1000))]
    city: String,
    #[validate(length(min = 1, max = 1000))]
    country: String,
}

async fn create_city(
    db: Extension<PgPool>,
    Json(req): Json<CreateCity>,
) -> Result<Json<City>, Error> {
    req.validate()?;
    let user_id = req.auth.verify(&*db).await?;

    db::City::insert(db, user.id, &req.city, &req.country).await?;
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
            user: v.user,
            city: v.city,
            country: v.country,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

/// Returns posts in descending chronological order.
async fn get_cities(db: Extension<PgPool>) -> Result<Json<Vec<City>>> {
    // Note: normally you'd want to put a `LIMIT` on this as well,
    // though that would also necessitate implementing pagination.
    let posts = sqlx::query_as!(
        Post,
        // language=PostgreSQL
        r#"
            select post_id, username, content, created_at
            from post
            inner join "user" using (user_id)
            order by created_at desc
        "#
    )
    .fetch_all(&*db)
    .await?;

    Ok(Json(posts))
}
