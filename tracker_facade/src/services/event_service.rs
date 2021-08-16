use crate::dtos::event_dto::EventDto;
use crate::errors::{ServiceError};

pub fn process_event_async(dto: EventDto) -> Result<(), ServiceError> {

    // REDIS CHECK DUBLICATES:
    
    // SEND KAFKA MESSAGE:

    Ok(())
}