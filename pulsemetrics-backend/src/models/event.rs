use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use validator::Validate;

/// Event data model for ingestion
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Event {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,

    #[serde(default = "Utc::now")]
    pub time: DateTime<Utc>,

    #[validate(length(min = 1, max = 100))]
    pub project_id: String,

    #[validate(length(min = 1, max = 50))]
    pub event_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JsonValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 100))]
    pub user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// Batch of events for ingestion
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EventBatch {
    #[validate(length(min = 1, max = 1000))]
    #[validate]
    pub events: Vec<Event>,
}

impl EventBatch {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// Response for successful ingestion
#[derive(Debug, Serialize)]
pub struct IngestionResponse {
    pub accepted: usize,
    pub timestamp: DateTime<Utc>,
}

impl IngestionResponse {
    pub fn new(accepted: usize) -> Self {
        Self {
            accepted,
            timestamp: Utc::now(),
        }
    }
}