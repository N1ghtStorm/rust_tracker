use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;

use crate::{config, errors::KafkaSenderError, models::event::Event};

#[deprecated(since = "0.0", note = "Use batch sending")]
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


pub fn send_events(events_batch: &Vec<Event>) -> Result<(), KafkaSenderError>{
    let mut producer = match get_producer() {
                        Err(_) => return Err(KafkaSenderError{message: "error creating kafka producer".to_string()}),
                        Ok(p) => p
                    };

    let messages = Event::get_serialized_string_vec(events_batch);

    for message in messages {
        if let Err(e) = producer.send(&Record::from_value(config::APP_CONFIG.get_kafka_topic(), message.as_bytes())) {
            println!("{}", e);
            return Err(KafkaSenderError{message: "error sending to kafka".to_string()});
        }
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