use std::str::FromStr;

use super::conversation::Conversation;
use crate::{auth_service::claims::JwtClaims, server::AppState};
use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn conversation_routes(state: AppState) -> axum::Router<AppState> {
    axum::Router::new()
        // post requeset to send a message
        .route("/message", post(send_message_service))
        //post request to create a conversation
        .route("/", post(start_conversation_service))
        //gets all the messages in the conversation
        .route("/message", get(get_conversation_service))
        .with_state(state)
}

#[derive(Deserialize, Serialize)]
pub struct ConversationRequest {
    // this is the id of the user that the current logged in user is starting a conversation with
    // the current user or "sender_id" will be found in the JWT token
    receiver_id: String,
}

// for this you need to set the header AUTHORIZATION as the jwt stored in local storage after signin
// if successfull it will return the conversation_id, you can redirect the user to conversation/:conversation_id and get the conversation data
pub async fn start_conversation_service(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(conversation_request): Json<ConversationRequest>,
) -> impl IntoResponse {
    //extract authorization header
    // Get the JWT token from the authorization header. Frontend should set this header with the token received during sign in.
    let authorization_header = headers
        .get(AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok());

    let response = match authorization_header {
        Some(token) => {
            // Decode the JWT token
            match JwtClaims::decode(&token.to_string()) {
                Ok(claims) => {
                    let sender_id = claims.user_id;
                    let pool = &state.pool;

                    // Convert sender and receiver IDs to UUIDs
                    match Uuid::from_str(&conversation_request.receiver_id) {
                        Ok(receiver_id) => {
                            // Start a new conversation
                            match Conversation::start(pool, sender_id, receiver_id).await {
                                Ok(conversation_id) => {
                                    // Return the conversation ID to the frontend on success
                                    (StatusCode::OK, conversation_id.to_string()).into_response()
                                }
                                Err(e) => {
                                    tracing::error!("could not start conversation: {:?}", e);
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        "Could not start conversation",
                                    )
                                        .into_response()
                                }
                            }
                        }
                        _ => {
                            // Return an error if the sender or receiver ID is invalid
                            (StatusCode::BAD_REQUEST, "Invalid sender||receiver id").into_response()
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("could not decode jwt token in start conversation: {:?}", e);
                    // Return an error if the JWT token is invalid
                    (StatusCode::BAD_REQUEST, "Invalid jwt").into_response()
                }
            }
        }
        None => {
            // Return an error if the authorization header is missing
            (StatusCode::UNAUTHORIZED, "No authorization header").into_response()
        }
    };

    response
}

#[derive(Deserialize, Serialize)]
pub struct GetConversationRequest {
    conversation_id: String,
}

// For the frontend:
// Send a POST request to /conversation with a JSON body containing the conversation_id.
// Include the JWT in the Authorization header obtained during sign-in.
// On success, you'll receive a JSON representation of the conversation Vec<Message> see the message.rs to view the data structure.
// Handle 400 errors for invalid conversation IDs or JWTs, and 401 for missing authorization headers.
// 500 errors indicate server-side issues.
pub async fn get_conversation_service(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(conversation_request): Json<GetConversationRequest>,
) -> impl IntoResponse {
    // Get the JWT token from the authorization header
    let authorization_header = headers
        .get(AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok());

    let response = match authorization_header {
        Some(token) => {
            // Decode the JWT token
            match JwtClaims::decode(&token.to_string()) {
                Ok(_) => {
                    // Get the database pool from the application state
                    let pool = &state.pool;

                    // Convert the conversation ID from a string to a UUID
                    match Uuid::from_str(&conversation_request.conversation_id) {
                        Ok(conversation_id) => {
                            // Get the conversation from the database
                            match Conversation::get_all_messages(pool, conversation_id).await {
                                Ok(messages) => {
                                    // Return the conversation to the client
                                    match serde_json::to_string(&messages) {
                                        Ok(json_string) => {
                                            (StatusCode::OK, Json(json_string)).into_response()
                                        }
                                        Err(e) => {
                                            tracing::error!(
                                                "could not serialize conversation: {:?}",
                                                e
                                            );
                                            (
                                                StatusCode::INTERNAL_SERVER_ERROR,
                                                "Could not serialize conversation",
                                            )
                                                .into_response()
                                        }
                                    }
                                }
                                Err(e) => {
                                    // Log the error and return an internal server error response
                                    tracing::error!("could not get conversation: {:?}", e);
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        "Could not get conversation",
                                    )
                                        .into_response()
                                }
                            }
                        }
                        Err(_) => {
                            // Return a bad request response if the conversation ID is invalid
                            (StatusCode::BAD_REQUEST, "Invalid conversation id").into_response()
                        }
                    }
                }
                Err(e) => {
                    // Log the error and return a bad request response if the JWT is invalid
                    tracing::error!("could not decode jwt token in get conversation: {:?}", e);
                    (StatusCode::BAD_REQUEST, "Invalid jwt").into_response()
                }
            }
        }
        None => {
            // Return an unauthorized response if the authorization header is missing
            (StatusCode::UNAUTHORIZED, "No authorization header").into_response()
        }
    };

    response
}

#[derive(Deserialize, Serialize)]
pub struct SendMessageRequest {
    conversation_id: String,
    content: String,
}

// will return an error or OK if the message is sent
// you can get messages after sending the message to update the ui
pub async fn send_message_service(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(message_request): Json<SendMessageRequest>,
) -> impl IntoResponse {
    // Get the JWT token from the authorization header
    let authorization_header = headers
        .get(AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok());

    let response = match authorization_header {
        Some(token) => {
            // Decode the JWT token
            match JwtClaims::decode(&token.to_string()) {
                Ok(claims) => {
                    let sender_id = claims.user_id;
                    let pool = &state.pool;

                    // Convert sender and receiver IDs to UUIDs
                    match Uuid::from_str(&message_request.conversation_id) {
                        Ok(conversation_id) => {
                            // Send the message
                            match Conversation::send_message(
                                pool,
                                sender_id,
                                conversation_id,
                                &message_request.content,
                            )
                            .await
                            {
                                Ok(_) => (StatusCode::OK, "Message sent").into_response(),
                                Err(e) => {
                                    tracing::error!("could not send message: {:?}", e);
                                    (StatusCode::INTERNAL_SERVER_ERROR, "Could not send message")
                                        .into_response()
                                }
                            }
                        }
                        _ => {
                            // Return an error if the sender or receiver ID is invalid
                            (StatusCode::BAD_REQUEST, "Invalid sender||conversation id")
                                .into_response()
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("could not decode jwt token in send message: {:?}", e);
                    // Return an error if the JWT token is invalid
                    (StatusCode::BAD_REQUEST, "Invalid jwt").into_response()
                }
            }
        }
        None => {
            // Return an error if the authorization header is missing
            (StatusCode::UNAUTHORIZED, "No authorization header").into_response()
        }
    };

    response
}
