use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};

use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;
use std::fmt::Write;

use crate::{config, errors::KafkaSenderError, models::event::Event};

pub fn send_event(event: Event) -> Result<(), KafkaSenderError>{
    // let producer = get_producer();
    // let message = event.get_serialized_string();

    // let record = BaseRecord::to(config::APP_CONFIG.get_kafka_topic())
    //                                         .key("1")
    //                                         .payload(&message);

    // match producer.send(record) {
    //     Err((ke, rec)) => Err(KafkaSenderError {message: "error sending to kafka".to_string()}),
    //     Ok(()) => Ok(()) 
    // }

    let mut producer =
    Producer::from_hosts(vec!("localhost:9092".to_owned()))
                    .with_ack_timeout(Duration::from_secs(1))
                    .with_required_acks(RequiredAcks::One)
                    .create()
                    .unwrap();

    let buf = String::from("{ python pravit mirom }");

    if let Err(e) = producer.send(&Record::from_value("facade-inbox", buf.as_bytes())) {
        println!("{}", e);
        return Err(KafkaSenderError{message: "error sending to kafka".to_string()});
    }
    // for _ in 0..10 {
    //     //let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
    //     producer.send(&Record::from_value("facade-inbox", buf.as_bytes())).unwrap();
    //     //buf.clear();
    // }

    Ok(())
}

fn get_producer() -> BaseProducer {
    ClientConfig::new()
                .set("bootstrap.servers", config::APP_CONFIG.get_kafka_brokers())
                .create()
                .expect("invalid producer config")
}