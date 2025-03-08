use crate::server::AppState;
use axum::{
    extract::{Form, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::post,
    Json,
};
use serde::{Deserialize, Serialize};

pub fn conversation_routes(state: AppState) -> axum::Router<AppState> {
    axum::Router::new()
        .route("/conversation/message", post(send_message_service))
        .route("/convesation", post(start_conversation_service))
        .with_state(state)
}

#[derive(Deserialize, Serialize)]
pub struct ConversationRequest {
    sender_id: String,
    receiver_id: String,
}

pub async fn start_conversation_service(
    State(state): State<AppState>,
    Json(ConversationRequest): Json<ConversationRequest>,
) -> impl IntoResponse {
    todo!()
}

pub async fn send_message_service() -> impl IntoResponse {
    todo!()
}
