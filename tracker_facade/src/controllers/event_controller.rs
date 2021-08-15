use actix_web::{post, HttpResponse, Responder};
use serde_json;
use crate::controllers::dtos::event_dto::{EventDto};

#[post("/eventv1")]
pub async fn post_event_async(req_body: String) -> impl Responder {
    let deser_result: serde_json::Result<EventDto> = serde_json::from_str(&req_body);
    let event_dto = match deser_result {
        Err(_) => return HttpResponse::BadRequest().body("Bad event body"),
        Ok(e) => e
    };

    HttpResponse::NotFound().body("
        NOT IMPLEMETED
    ")
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