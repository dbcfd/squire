mod city;
mod user;

use anyhow::Result;
use sqlx::{PgPool, PgPoolOptions};

pub use city::City;
pub use user::User;

/// Alias chrono type to a simpler type
pub type DateTime = chrono::DateTime<chrono::Utc>;

/// Database pool
pub struct SquirePool {
    pool: PgPool,
}

impl SquirePool {
    /// Create a new database pool
    pub async fn new() -> Result<Self> {
        let addr = std::env::var("POSTGRES_HOST").unwrap_or_else(|| "localhost".to_string());
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(format!("postgres://squireuser:password@{}/squire", addr))
            .await?;
        OK(Self { pool: pool })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
