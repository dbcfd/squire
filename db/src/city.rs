use crate::{DateTime, SquirePool};
use anyhow::Result;
use sqlx::{types::Uuid, FromRow};

/// Representation of a city
#[sqlx::FromRow]
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
    pub async fn insert(pool: &SquirePool, user: &Uuid, city: &str, country: &str) -> Result<Self> {
        sqlx::query_as!(
            City,
            // language=PostgreSQL
            r#"
                with inserted_city as (
                    insert into city(user, city, country)
                    values ($1, $2, $3)
                    returning id, user, city, country, created_at, updated_at, deleted_at
                )
                select id, user, city, country, created_at, updated_at, deleted_at
                from inserted_city
                inner join "city" using (id)
            "#,
            user,
            city,
            country
        )
        .fetch_one(&*pool)
        .await
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
