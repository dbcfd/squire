use crate::{DateTime, SquirePool};
use anyhow::Result;
use sqlx::types::Uuid;

/// Representation of a user
#[derive(sqlx::FromRow)]
pub struct User {
    id: Uuid,
    email: String,
    name: String,
    hashed_password: String,
    created_at: DateTime,
    updated_at: DateTime,
    deleted_at: Option<DateTime>,
}

impl User {
    pub async fn insert(
        pool: &SquirePool,
        email: &str,
        name: &str,
        hashed_password: &str,
    ) -> Result<Self> {
        sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
                insert into squire.user(email, name, hashed_password)
                values ($1, $2, $3)
                returning id, email, hashed_password, created_at, updated_at, deleted_at
            "#,
            email,
            name,
            hashed_password,
        )
        .fetch_one(&*pool.pool)
        .await?
    }

    pub async fn get(pool: &SquirePool, email: &str) -> Result<Option<Self>> {
        sqlx::query_as!(
            User,
            r#"select id, email, name, hashed_password, created_at, updated_at, deleted_at from squire.user where email = $1 and deleted_at is null"#,
            email
        )
        .fetch_optional(&*pool.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_insert_user() {
        let pool = SquirePool::new().await.unwrap();

        let user = User::insert(&pool, "insert_user@test.com", "hashed_password")
            .await
            .unwrap();
        assert_eq!(user.email, "insert_user@test.com");

        let user = User::get(&pool, "insert_user@test.com").await.unwrap();
        assert!(user.is_some());
    }
}
