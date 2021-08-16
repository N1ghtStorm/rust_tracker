use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EventDto {
    pub uuid: String,
    pub action: String,
    pub date: String,
    pub button_name: String,
}