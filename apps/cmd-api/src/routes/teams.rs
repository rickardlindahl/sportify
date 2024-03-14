use axum::{extract, http::StatusCode, Json};

use serde::{Deserialize, Serialize};
use sportify_protobuf::{protos::teams::CreateTeam, serialize_add_team};
use uuid::Uuid;

use crate::config::Config;

pub async fn create_team(
    extract::State(config): extract::State<Config>,
    extract::Json(payload): extract::Json<CreateTeamPayload>,
) -> (StatusCode, Json<Response>) {
    let producer = config.kafka_producer;

    let data = CreateTeam::from(payload);

    let send_result = producer
        .produce(
            &config.kafka_topic_teams,
            &data.id.to_string(),
            &serialize_add_team(&data),
        )
        .await;

    match send_result {
        Ok((partition, offset)) => {
            println!(
                "Produced message to partition {} with offset {}",
                partition, offset
            );

            (
                StatusCode::CREATED,
                Json(Response::Success(IdResponse {
                    id: Uuid::parse_str(&data.id).unwrap(),
                })),
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
pub struct CreateTeamPayload {
    name: String,
    founded: u32,
    stadium: String,
    city: String,
    country: String,
}

impl From<CreateTeamPayload> for CreateTeam {
    fn from(payload: CreateTeamPayload) -> Self {
        CreateTeam {
            id: Uuid::new_v4().to_string(),
            name: payload.name,
            founded: payload.founded,
            stadium: payload.stadium,
            city: payload.city,
            country: payload.country,
        }
    }
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
