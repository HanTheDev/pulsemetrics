use sqlx::PgPool;
use std::sync::Arc;

use crate::config::Config;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(db: PgPool, config: Config) -> Self {
        Self {
            db,
            config: Arc::new(config),
        }
    }
}

// Re-export commonly used items
pub mod config;
pub mod db;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;

pub use models::{AppError, AppResult};