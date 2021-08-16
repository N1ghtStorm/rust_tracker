use uuid::Uuid;

pub struct Event {
    pub uuid: Uuid,
    pub action: String,
    pub date: String,
    pub button_name: String,
}