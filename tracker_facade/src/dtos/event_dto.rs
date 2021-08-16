use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventDto {
    pub uuid: String,
    pub action: String,
    pub date: String,
    pub button_name: String,
}