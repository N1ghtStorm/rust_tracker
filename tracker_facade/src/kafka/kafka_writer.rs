use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};

use crate::{config, errors::KafkaSenderError, models::event::Event};

pub fn send_event(event: Event) -> Result<(), KafkaSenderError>{
    let producer = get_producer();
    let message = event.get_serialized_string();

    let record = BaseRecord::to(config::APP_CONFIG.get_kafka_topic())
                                            .key("")
                                            .payload(&message);

    match producer.send(record) {
        Err(_) => Err(KafkaSenderError {message: "error sending to kafka".to_string()}),
        Ok(()) => Ok(()) 
    }
}

fn get_producer() -> BaseProducer {
    ClientConfig::new()
                .set("bootstrap.servers", config::APP_CONFIG.get_kafka_brokers())
                .create()
                .expect("invalid producer config")
}