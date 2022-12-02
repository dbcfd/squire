use crate::{DateTime, Error, SquirePool};
use sqlx::types::Uuid;

/// Representation of a user
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub hashed_password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

impl User {
    pub async fn insert(
        pool: &SquirePool,
        email: &str,
        name: &str,
        hashed_password: &str,
    ) -> Result<Self, Error> {
        let res = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
                insert into squire.user(email, name, hashed_password)
                values ($1, $2, $3)
                returning id, email, name, hashed_password, created_at, updated_at, deleted_at
            "#,
            email,
            name,
            hashed_password,
        )
        .fetch_one(&pool.pool)
        .await?;
        Ok(res)
    }

    pub async fn get(pool: &SquirePool, email: &str) -> Result<Option<Self>, Error> {
        let res = sqlx::query_as!(
            User,
            r#"select id, email, name, hashed_password, created_at, updated_at, deleted_at from squire.user where email = $1 and deleted_at is null"#,
            email
        )
        .fetch_optional(&pool.pool)
        .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_insert_user() {
        let pool = SquirePool::new().await.unwrap();

        let user = User::insert(&pool, "insert_user@test.com", "some name", "hashed_password")
            .await
            .unwrap();
        assert_eq!(user.email, "insert_user@test.com");

        let user = User::get(&pool, "insert_user@test.com").await.unwrap();
        assert!(user.is_some());
    }
}
