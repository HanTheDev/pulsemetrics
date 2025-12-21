use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

use crate::{
    models::{AppError, AppResult, EventBatch, IngestionResponse},
    AppState,
};

/// Ingest a batch of events
/// 
/// Accepts up to 1000 events per request
/// Returns 202 Accepted immediately (async processing)
pub async fn ingest_events(
    State(state): State<AppState>,
    Json(batch): Json<EventBatch>,
) -> AppResult<(StatusCode, Json<IngestionResponse>)> {
    // Validate batch
    batch.validate()?;

    // Check batch size
    if batch.len() > state.config.app.max_batch_size {
        return Err(AppError::BadRequest(format!(
            "Batch size {} exceeds maximum of {}",
            batch.len(),
            state.config.app.max_batch_size
        )));
    }

    tracing::debug!("Received batch of {} events", batch.len());

    // Insert events into database
    insert_events(&state, &batch).await?;

    tracing::info!("Successfully ingested {} events", batch.len());

    Ok((
        StatusCode::ACCEPTED,
        Json(IngestionResponse::new(batch.len())),
    ))
}

/// Insert events into the database
async fn insert_events(state: &AppState, batch: &EventBatch) -> AppResult<()> {
    // Build bulk insert query
    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO events (id, time, project_id, event_type, properties, user_id, session_id, value) "
    );

    query_builder.push_values(&batch.events, |mut b, event| {
        b.push_bind(event.id)
            .push_bind(event.time)
            .push_bind(&event.project_id)
            .push_bind(&event.event_type)
            .push_bind(&event.properties)
            .push_bind(&event.user_id)
            .push_bind(event.session_id)
            .push_bind(event.value);
    });

    let query = query_builder.build();

    // Execute the query
    query.execute(&state.db).await?;

    Ok(())
}