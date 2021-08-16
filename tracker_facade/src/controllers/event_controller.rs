use actix_web::{post, HttpResponse, Responder};
use serde_json;
use crate::{dtos::event_dto::{EventDto}, services};

#[post("/eventv1")]
pub async fn post_event_async(req_body: String) -> impl Responder {
    let deser_result: serde_json::Result<EventDto> = serde_json::from_str(&req_body);
    let event_dto = match deser_result {
        Err(_) => return HttpResponse::BadRequest().body("Bad event body"),
        Ok(e) => e
    };

    if let Err(err) = services::event_service::process_event_async(event_dto) {
        return HttpResponse::InternalServerError().body(format!("error has occured {}", err.message))
    }

    HttpResponse::Ok().body("")
}

#[post("/eventsv1")]
pub async fn post_events_async(req_body: String) -> impl Responder {
    let deser_result: serde_json::Result<Vec<EventDto>> = serde_json::from_str(&req_body);
    let events_dto = match deser_result {
        Err(_) => return HttpResponse::BadRequest().body("Bad event array body"),
        Ok(ve) => ve
    };

    HttpResponse::NotFound().body("
        NOT IMPLEMETED
    ")
}