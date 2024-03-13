use axum::{extract, http::StatusCode, Json};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;

pub async fn add_team(
    extract::State(config): extract::State<Config>,
    extract::Json(payload): extract::Json<Team>,
) -> (StatusCode, Json<Response>) {
    let id = Uuid::new_v4();
    let id_string = id.to_string();

    let team_with_id = TeamWithId { id, data: payload };

    let producer = config.kafka_producer;

    let buffer = serde_json::to_string(&team_with_id).unwrap();

    let send_result = producer
        .produce(&config.kafka_topic_teams, &id_string, buffer.as_bytes())
        .await;

    match send_result {
        Ok((partition, offset)) => {
            println!(
                "Produced message to partition {} with offset {}",
                partition, offset
            );

            (
                StatusCode::CREATED,
                Json(Response::Success(IdResponse { id })),
            )
        }
        Err(_) => {
            eprintln!("Error while producing message");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::Error(MessageResponse {
                    message: "Error while producing message".to_string(),
                })),
            )
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Team {
    name: String,
    founded: u16,
    stadium: String,
    city: String,
    country: String,
}

#[derive(Serialize)]
pub struct TeamWithId {
    id: Uuid,
    data: Team,
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
