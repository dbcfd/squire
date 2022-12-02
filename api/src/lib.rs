mod auth;
mod city;
mod error;
mod password;
mod user;

pub use db::SquirePool;
pub use error::Error;
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

use anyhow::Context;
use axum::{Extension, Router};

pub fn app(db: SquirePool) -> Router {
    Router::new()
        .merge(user::router())
        .merge(city::router())
        .layer(Extension(db))
}

pub async fn serve(db: SquirePool) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .context("failed to serve API")
}
