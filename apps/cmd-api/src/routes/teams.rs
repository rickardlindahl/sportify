use axum::{extract::rejection::JsonRejection, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn add_team(payload: Result<Json<Team>, JsonRejection>) -> (StatusCode, Json<Response>) {
    match payload {
        Ok(_payload) => {
            let response = IdResponse { id: Uuid::new_v4() };

            (StatusCode::CREATED, Json(Response::Success(response)))
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            let message = MessageResponse {
                message: "Missing Content-Type: application/json header".into(),
            };

            (StatusCode::BAD_REQUEST, Json(Response::Error(message)))
        }
        Err(JsonRejection::JsonDataError(_)) => {
            let message = MessageResponse {
                message: "Couldn't deserialize the body into the target type".into(),
            };

            (StatusCode::BAD_REQUEST, Json(Response::Error(message)))
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            // Syntax error in the body
            let message = MessageResponse {
                message: "Syntax error in the body".into(),
            };

            (StatusCode::BAD_REQUEST, Json(Response::Error(message)))
        }
        Err(JsonRejection::BytesRejection(_)) => {
            // Failed to extract the request body
            let message = MessageResponse {
                message: "Failed to extract the request body".into(),
            };

            (StatusCode::BAD_REQUEST, Json(Response::Error(message)))
        }
        Err(_) => {
            // `JsonRejection` is marked `#[non_exhaustive]` so match must include a catch-all case.
            let message = MessageResponse {
                message: "Unknown error".into(),
            };
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::Error(message)),
            )
        }
    }
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
