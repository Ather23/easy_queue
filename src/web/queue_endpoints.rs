use std::collections::VecDeque;
use axum::{ extract::{ Json, State }, http::StatusCode, response::Response };
use axum::extract::Path;
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::AppState;

use super::response::{ ApiErrorResponse, ResponseHandler };

pub async fn insert_message(
    State(state): State<AppState>,
    Path(queue_name): Path<String>,
    Json(payload): Json<InsertMessage>
) -> Response {
    info!("Insert message call");

    if let Ok(mut queues) = state.stream_hashmap.lock() {
        let q = queues.hashmap.entry(queue_name.clone()).or_insert_with(VecDeque::new);
        q.push_back(payload.message);
        return ResponseHandler::<String>
            ::new("Message inserted".to_string())
            .response(&StatusCode::NO_CONTENT);
    }
    return ResponseHandler::<String>
        ::new("Unable to add queue".to_string())
        .response(&StatusCode::INTERNAL_SERVER_ERROR);
}

pub async fn pop_message(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> Response {
    info!("Popping message");

    if let Ok(mut stream) = state.stream_hashmap.lock() {
        let q = stream.hashmap.entry(queue_name).or_insert_with(VecDeque::new);
        if let Some(data) = q.pop_front() {
            return ResponseHandler::<String>::new(data).response(&StatusCode::OK);
        } else {
            return ResponseHandler::<Vec<String>>
                ::new(Vec::<String>::new())
                .response(&StatusCode::OK);
        }
    } else {
        return ResponseHandler::<String>
            ::new("Unable to lock queue".to_string())
            .response(&StatusCode::INTERNAL_SERVER_ERROR);
    }
}

pub async fn get_queue_msg_count(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> Response {
    info!("Getting message count");

    if let Some(queue) = get_queue_by_name(state, queue_name.clone()).await {
        return ResponseHandler::<MessageCount>
            ::new(MessageCount {
                count: queue.len(),
            })
            .response(&StatusCode::OK);
    }

    return ResponseHandler::<ApiErrorResponse>
        ::new(ApiErrorResponse {
            msg: format!("Queue {queue_name} could not be found"),
        })
        .response(&StatusCode::NOT_FOUND);
}

pub async fn get_all_queue_messages(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> Response {
    info!("Getting all queue messages");
    if let Ok(map) = state.stream_hashmap.lock() {
        if let Some(queue) = map.hashmap.get(&queue_name) {
            return ResponseHandler::<Vec<String>>
                ::new(Vec::from(queue.to_owned()))
                .response(&StatusCode::OK);
        }
    }

    return ResponseHandler::<ApiErrorResponse>
        ::new(ApiErrorResponse {
            msg: format!("Queue {queue_name} could not be found"),
        })
        .response(&StatusCode::NOT_FOUND);
}

async fn get_queue_by_name(state: AppState, queue_name: String) -> Option<Vec<String>> {
    if let Ok(map) = state.stream_hashmap.lock() {
        if let Some(queue) = map.hashmap.get(&queue_name) {
            return Some(Vec::from(queue.to_owned()));
        }
    }
    None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertMessage {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageCount {
    count: usize,
}
