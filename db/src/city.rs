use crate::{DateTime, Error, SquirePool};
use sqlx::types::Uuid;

/// Representation of a city
#[derive(sqlx::FromRow)]
pub struct City {
    id: Uuid,
    user: Uuid,
    city: String,
    country: String,
    created_at: DateTime,
    updated_at: DateTime,
    deleted_at: Option<DateTime>,
}

impl City {
    pub async fn insert(
        pool: &SquirePool,
        user: &Uuid,
        city: &str,
        country: &str,
    ) -> Result<Self, Error> {
        sqlx::query_as!(
            City,
            // language=PostgreSQL
            r#"
                insert into squire.city(user, city, country)
                values ($1, $2, $3)
                returning id, user, city, country, created_at, updated_at, deleted_at
            "#,
            user,
            city,
            country
        )
        .fetch_one(pool.pool)
        .await
        .map_err(Error::from)
    }

    pub async fn get(pool: &SquirePool, user: &Uuid) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(
            City,
            r#"select id, user, city, country, created_at, updated_at, deleted_at from squire.city where user = $1 and deleted_at is null"#,
            user
        )
        .fetch(pool.pool)
        .await.map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_insert_city() {
        let pool = SquirePool::new().await.unwrap();

        let user = User::insert(&pool, "insert_user@test.com", "hashed_password")
            .await
            .unwrap();
        assert_eq!(user.email, "insert_user@test.com");

        let city = City::insert(&pool, user.id, "some_town", "some_country").await?;
        assert_eq!(city.user, user.id);
    }
}
