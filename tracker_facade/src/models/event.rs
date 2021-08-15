use chrono::{DateTime, Utc};
use uuid::Uuid;

struct Event {
    uuid: Uuid,
    action: String,
    date: DateTime<Utc>,
    buttonName: String,
}