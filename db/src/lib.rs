mod city;
mod error;
mod user;

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub use city::City;
pub use error::Error;
pub use sqlx::types::Uuid;
pub use user::User;

/// Alias chrono type to a simpler type
pub type DateTime = time::OffsetDateTime;

/// Database pool
#[derive(Clone)]
pub struct SquirePool {
    //TODO implement deref
    pub(crate) pool: PgPool,
}

impl SquirePool {
    /// Create a new database pool
    pub async fn new() -> Result<Self, Error> {
        let addr = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&format!("postgres://squireuser:password@{}/squire", addr))
            .await?;
        Ok(Self { pool: pool })
    }
}
