pub mod error;
pub mod event;

pub use error::{AppError, AppResult};
pub use event::{Event, EventBatch, IngestionResponse};