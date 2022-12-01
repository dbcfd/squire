use crate::{DateTime, SquirePool};
use anyhow::Result;
use sqlx::FromRow;

/// Representation of a user
#[sqlx::FromRow]
pub struct User {
    id: Uuid,
    email: String,
    hashed_password: String,
    created_at: DateTime,
    updated_at: DateTime,
    deleted_at: Option<DateTime>,
}

impl User {
    pub async fn insert(
        pool: &SquirePool,
        email: &str,
        hashed_password: &str,
        salt: &str,
    ) -> Result<Self> {
        sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
                with inserted_user as (
                    insert into user(email, hashed_password, salt)
                    values ($1, $2, $3)
                    returning id, email, hashed_password, salt, created_at, updated_at, deleted_at
                )
                select id, email, hashed_password, salt, created_at, updated_at, deleted_at
                from inserted_user
                inner join "user" using (id)
            "#,
            email,
            hashed_password,
            salt
        )
        .fetch_one(&*pool)
        .await?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_insert_user() {
        let pool = SquirePool::new().await.unwrap();

        let user = User::insert(&pool, "insert_user@test.com", "hashed_password", "salt").await?;
        assert_eq!(user.email, "insert_user@test.com");
    }
}
