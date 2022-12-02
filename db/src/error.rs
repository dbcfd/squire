/// An API-friendly error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    pub fn constraint_error(&self) -> bool {
        if let Error::Sqlx(sqlx::Error::Database(e)) = self {
            e.constraint()
                .map(|s| s.contains("key"))
                .unwrap_or_else(|| false)
        } else {
            false
        }
    }
}
