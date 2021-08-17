use crate::dtos::event_dto::EventDto;
use crate::errors::{ServiceError};
use crate::kafka_sending::kafka_writer::{send_event};
use crate::models::event::Event;

pub fn process_event_async(dto: EventDto) -> Result<(), ServiceError> {
    // REDIS CHECK DUBLICATES:
    
    let event = match Event::new_from_event_dto(dto) {
        Err(err) => return Err(ServiceError {message: err.message}),
        Ok(ev) => ev
    };

    // SEND KAFKA MESSAGE:
    if let Err(err) = send_event(event) {
        return Err(ServiceError {message:err.message});
    }

    Ok(())
}