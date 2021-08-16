use std::fs::File;
use std::io::Read;
use serde_json;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_CONFIG: ApplicationConfig = get_application_config();
}

const APPSETTINGS: &str = "appsettings.json";

pub fn get_application_config() -> ApplicationConfig {
    let mut file = File::open(APPSETTINGS).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let deser_result: serde_json::Result<ApplicationConfig> = serde_json::from_str(&data);

    match deser_result {
        Err(e) => panic!("Error reading {}: {}", APPSETTINGS, e),
        Ok(ac) => ac
    }
}

// MAIN STRUCT FOR APPLICATION CONFIGURATION
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApplicationConfig {
    kafka: KafkaConfig,
    redis: RedisConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct KafkaConfig {
    brokers: String,
    consumer_group: String,
    topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct RedisConfig {
    addr: String
}


impl ApplicationConfig {
    pub fn get_kafka_brokers(&self) -> &String {
        return &self.kafka.brokers;
    }

    pub fn get_kafka_consumer_group(&self) -> &String {
        return &self.kafka.consumer_group;
    }

    pub fn get_kafka_topic(&self) -> &String {
        return &self.kafka.topic;
    }

    pub fn get_redis_addr(&self) -> &String {
        return &self.redis.addr;
    }
}