use std::fs::File;
use std::io::Read;
use serde_json;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_CONFIG: ApplicationConfig = get_application_config();
}

const APPSETTINGS: &str = "./src/appsettings.json";

pub fn get_application_config() -> ApplicationConfig {
    let mut file = File::open(APPSETTINGS).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let deser_result: serde_json::Result<ApplicationConfig> = serde_json::from_str(&data);

    match deser_result {
        Err(e) => panic!("Error reading {}: {}", APPSETTINGS, e),
        Ok(a) => a
    }
}

// MAIN STRUCT FOR APPLICATION CONFIGURATION
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApplicationConfig {
    pub kafka: KafkaConfig,
    pub redis: RedisConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KafkaConfig {
    pub brokers: String,
    pub consumer_group: String,
    pub topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RedisConfig {
    pub addr: String
}