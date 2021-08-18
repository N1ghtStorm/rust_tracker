use uuid::Uuid;
use chrono;
use crate::{dtos::event_dto::EventDto, errors::EventError};
use serde::{Serialize};
use serde_json;


/// MAIN DOMAIN:
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub uuid: String,
    pub action: String,
    pub date: String,
    pub button_name: String,
    pub server_timestamp: String,
}

impl Event {
    /// Construct from eventdto
    pub fn new_from_event_dto(dto: EventDto) -> Result<Self, EventError> {
        let string_server_timestamp = chrono::offset::Local::now().to_string();

        if let Err(_) = Uuid::parse_str(&dto.uuid) {
            return Err(EventError{message: "error parsing uuid".to_string()});
        }

        Ok(Event {uuid: dto.uuid, action: dto.action, date: dto.date, button_name:dto.button_name, server_timestamp: string_server_timestamp})
    }

    /// Construct events from eventdtos
    pub fn new_eventvec_from_dto(dtos: Vec<EventDto>) -> Result<Vec<Self>, EventError> {
        let string_server_timestamp = chrono::offset::Local::now().to_string();

        // Check if uuids are valid
        for dto in &dtos {
            if let Err(_) = Uuid::parse_str(&dto.uuid) {
                return Err(EventError{message: "error parsing uuid".to_string()});
            }
        }

        let events = dtos.into_iter()
                                    .map(|dto| {      
                                        Event {
                                            uuid: dto.uuid, 
                                            action: dto.action, 
                                            date: dto.date, 
                                            button_name:dto.button_name, 
                                            server_timestamp: string_server_timestamp.clone(),
                                        }
                                    })
                                    .collect::<Vec<Self>>();
        Ok(events)
    }


    // EVENT BY ITSELF MUST KNOW HOW TO BE SERIALIZED:

    /// Get serialized string from single event
    pub fn get_serialized_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Get serialized strings from multiples events
    pub fn get_serialized_string_vec(event_vec: &Vec<Event>) -> Vec<String> {
        event_vec.iter()
                 .map(|x| x.get_serialized_string())
                 .collect::<Vec<String>>()
    }
}