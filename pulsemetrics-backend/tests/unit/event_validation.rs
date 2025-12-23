use pulsemetrics_backend::models::{Event, EventBatch};
use uuid::Uuid;
use validator::Validate;

#[test]
fn test_valid_event() {
    let event = Event {
        id: Uuid::new_v4(),
        time: chrono::Utc::now(),
        project_id: "test-project".to_string(),
        event_type: "page_view".to_string(),
        properties: None,
        user_id: None,
        session_id: None,
        value: None,
    };

    assert!(event.validate().is_ok());
}

#[test]
fn test_invalid_event_empty_project_id() {
    let event = Event {
        id: Uuid::new_v4(),
        time: chrono::Utc::now(),
        project_id: "".to_string(), // Invalid: empty
        event_type: "page_view".to_string(),
        properties: None,
        user_id: None,
        session_id: None,
        value: None,
    };

    assert!(event.validate().is_err());
}

#[test]
fn test_invalid_event_long_project_id() {
    let event = Event {
        id: Uuid::new_v4(),
        time: chrono::Utc::now(),
        project_id: "a".repeat(101), // Invalid: too long
        event_type: "page_view".to_string(),
        properties: None,
        user_id: None,
        session_id: None,
        value: None,
    };

    assert!(event.validate().is_err());
}

#[test]
fn test_valid_event_batch() {
    let events = vec![
        Event {
            id: Uuid::new_v4(),
            time: chrono::Utc::now(),
            project_id: "test".to_string(),
            event_type: "click".to_string(),
            properties: None,
            user_id: None,
            session_id: None,
            value: None,
        },
    ];

    let batch = EventBatch::new(events);
    assert!(batch.validate().is_ok());
}

#[test]
fn test_invalid_event_batch_empty() {
    let batch = EventBatch::new(vec![]);
    assert!(batch.validate().is_err());
}

#[test]
fn test_invalid_event_batch_too_large() {
    let events = (0..1001)
        .map(|_| Event {
            id: Uuid::new_v4(),
            time: chrono::Utc::now(),
            project_id: "test".to_string(),
            event_type: "click".to_string(),
            properties: None,
            user_id: None,
            session_id: None,
            value: None,
        })
        .collect();

    let batch = EventBatch::new(events);
    assert!(batch.validate().is_err());
}