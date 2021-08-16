use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;

use crate::{config, errors::KafkaSenderError, models::event::Event};

pub fn send_event(event: Event) -> Result<(), KafkaSenderError>{
    let mut producer = match get_producer() {
                        Err(_) => return Err(KafkaSenderError{message: "error sending to kafka".to_string()}),
                        Ok(p) => p
                    };

    let message = event.get_serialized_string();

    if let Err(e) = producer.send(&Record::from_value(config::APP_CONFIG.get_kafka_topic(), message.as_bytes())) {
        println!("{}", e);
        return Err(KafkaSenderError{message: "error sending to kafka".to_string()});
    }

    Ok(())
}

fn get_producer() -> Result<Producer, kafka::Error> {
    // TODO: REALLOC PROBLEM
    Producer::from_hosts(vec!(config::APP_CONFIG.get_kafka_brokers().clone()))
                    .with_ack_timeout(Duration::from_secs(5))
                    .with_required_acks(RequiredAcks::One)
                    .create()
}