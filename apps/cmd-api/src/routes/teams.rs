use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn add_team(
    extract::Json(_payload): extract::Json<Team>,
) -> (StatusCode, Json<Response>) {
    let response = IdResponse { id: Uuid::new_v4() };

    (StatusCode::CREATED, Json(Response::Success(response)))
}

#[derive(Deserialize)]
pub struct Team {
    name: String,
    founded: u16,
    stadium: String,
    city: String,
    country: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
    Success(IdResponse),
    Error(MessageResponse),
}

#[derive(Serialize)]
pub struct IdResponse {
    id: Uuid,
}

#[derive(Serialize)]
pub struct MessageResponse {
    message: String,
}
