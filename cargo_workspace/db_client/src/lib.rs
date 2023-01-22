#[macro_use]
extern crate lazy_static;

use std::path::Path;

use anyhow::{Context as _, Result};

mod context;
pub mod model;
pub use context::Context;
mod config;
use config::DATABASE_URL;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Postgres};

pub type Pool = sqlx::Pool<Postgres>;
pub type PooledConnection = PoolConnection<Postgres>;

/// Connect to the database and run any migrations. This will
/// log the migration ouput to stdout.
pub async fn connect() -> Result<Pool> {
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(*DATABASE_URL)
        .await
        .with_context(|| "Connecting to the database failed")?;

    // Run migrations
    sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .with_context(|| "Failed to find migrations")?
        .set_ignore_missing(true)
        .run(&pool)
        .await
        .with_context(|| "Running database migrations failed")?;

    Ok(pool)
}
