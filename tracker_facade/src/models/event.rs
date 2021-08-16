use uuid::Uuid;
use chrono;
use crate::{dtos::event_dto::EventDto, errors::EventError};
use serde::{Serialize};
use serde_json;


#[derive(Serialize)]
pub struct Event {
    pub uuid: String,
    pub action: String,
    pub date: String,
    pub button_name: String,
    pub server_timestamp: String,
}

impl Event {
    pub fn new_from_event_dto(dto: EventDto) -> Result<Self, EventError> {
        let string_server_timestamp = chrono::offset::Local::now().to_string();

        if let Err(_) = Uuid::parse_str(&dto.uuid) {
            return Err(EventError{message: "error parsing uuid".to_string()});
        }

        Ok(Event {uuid: dto.uuid, action: dto.action, date: dto.date, button_name:dto.button_name, server_timestamp: string_server_timestamp})
    }

    pub fn get_serialized_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}