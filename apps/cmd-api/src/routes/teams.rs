use axum::{extract, http::StatusCode, Json};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_kafka_producer() -> FutureProducer {
    let broker = std::env::var("KAFKA_BROKER").unwrap();
    let username = std::env::var("KAFKA_USERNAME").unwrap();
    let password = std::env::var("KAFKA_PASSWORD").unwrap();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "SCRAM-SHA-256")
        .set("sasl.username", username)
        .set("sasl.password", password)
        .create()
        .expect("Producer creation error");

    producer
}

fn get_timestamp_miliseconds(now: SystemTime) -> i64 {
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    (duration_since_epoch.as_secs() as i64) * 1000 + i64::from(duration_since_epoch.subsec_millis())
}

pub async fn add_team(extract::Json(payload): extract::Json<Team>) -> (StatusCode, Json<Response>) {
    let id = Uuid::new_v4();

    let team_with_id = TeamWithId { id, data: payload };

    let producer = get_kafka_producer();

    let buffer = serde_json::to_string(&team_with_id).unwrap();

    let topic = std::env::var("KAFKA_ADD_TEAM_TOPIC").unwrap();
    let key = std::env::var("KAFKA_ADD_TEAM_KEY").unwrap();

    let record = FutureRecord::to(&topic)
        .key(&key)
        .payload(buffer.as_bytes())
        .timestamp(get_timestamp_miliseconds(SystemTime::now()));

    let send_result = producer.send(record, Duration::from_secs(0)).await;
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
