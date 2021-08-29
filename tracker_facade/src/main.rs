use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use batching::batcher::{BatchConfig, Batcher};
use kafka_sending::kafka_writer::send_events;
use std::sync::{Arc, RwLock, Mutex};

pub mod models;
pub mod dtos;

pub mod controllers;
pub mod services;
pub mod kafka_sending;
pub mod errors;
pub mod batching;

// APPLICATION CONFIG MODULES
pub mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:8002";
    let a = config::get_application_config();

    println!("===========================================");
    
    println!("{:?}", a);

    let batch_config = BatchConfig::new(3, 2);
    let func = Arc::new(Mutex::new(send_events));
    let mut batcher = Arc::new(Batcher::new(batch_config, func.clone()));
    
    //let _ = batcher.start_batching();
    web::Data::new( AppState::new(batcher.clone()));

    // std::thread::spawn(move || {
    //     // let batch_config = BatchConfig::new(3, 2);
    //     // let func = Box::new(send_events);
    //     // let mut batcher = Batcher::new(batch_config, func);
    //     let a = batcher.clone();//.start_batching();
    // });

    // START HTTP SERVER WITH GLOBAL STATE
    HttpServer::new( move || {  
        App::new()
            .service(hi)
            .service(controllers::event_controller::post_event_async)
            .service(controllers::event_controller::post_events_async)
    })
    .bind(url)?
    .run()
    .await
}

struct AppState {
    batcher: Arc<Batcher<models::event::Event, errors::KafkaSenderError>>
}

impl AppState{
    pub fn new(batcher: Arc<Batcher<models::event::Event, errors::KafkaSenderError>>) -> Self {
        AppState{batcher}
    }
}

/// Healthcheck endpoint
#[get("/info")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("
        TRACKER FACADE IS RUNNING
    ")
}