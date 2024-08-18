use std::collections::VecDeque;

use axum::{ extract::{ Json, State }, http::StatusCode, response::{ IntoResponse, Response } };
use axum::extract::Path;
use serde::{ Deserialize, Serialize };

use crate::{ AppState, EasyQueue };

use super::response::{ ApiErrorResponse, ResponseHandler };

pub async fn insert_message(
    State(state): State<AppState>,
    Path(queue_name): Path<String>,
    Json(payload): Json<InsertMessage>
) -> Response {
    let mut queues = state.queues.lock().unwrap();
    let q = queues.queue.entry(queue_name.clone()).or_insert_with(VecDeque::new);
    q.push_back(payload.message);

    return ResponseHandler::<String>
        ::new("Message inserted".to_string())
        .response(&StatusCode::NO_CONTENT);
}

pub async fn pop_message(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> Response {
    let mut queues = state.queues.lock().unwrap();
    let q = queues.queue.entry(queue_name).or_insert_with(VecDeque::new);
    if let Some(data) = q.pop_front() {
        return ResponseHandler::<String>::new(data).response(&StatusCode::OK);
    } else {
        return ResponseHandler::<Vec<String>>::new(Vec::<String>::new()).response(&StatusCode::OK);
    }
}

pub async fn get_queue_msg_count(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> Response {
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
    let queues: std::sync::MutexGuard<EasyQueue> = state.queues.lock().unwrap();
    if let Some(queue) = queues.queue.get(&queue_name) {
        return ResponseHandler::<Vec<String>>
            ::new(Vec::from(queue.to_owned()))
            .response(&StatusCode::OK);
    }

    return ResponseHandler::<ApiErrorResponse>
        ::new(ApiErrorResponse {
            msg: format!("Queue {queue_name} could not be found"),
        })
        .response(&StatusCode::NOT_FOUND);
}

async fn get_queue_by_name(state: AppState, queue_name: String) -> Option<Vec<String>> {
    let queues: std::sync::MutexGuard<EasyQueue> = state.queues.lock().unwrap();
    if let Some(queue) = queues.queue.get(&queue_name) {
        return Some(Vec::from(queue.to_owned()));
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
