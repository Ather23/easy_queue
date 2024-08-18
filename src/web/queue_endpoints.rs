use std::collections::VecDeque;

use axum::{
    body::Body,
    extract::{ Json, State },
    http::StatusCode,
    response::{ IntoResponse, Response },
};
use axum::extract::Path;
use serde::{ Deserialize, Serialize };

use crate::{ AppState, EasyQueue };

pub async fn insert_message(
    State(state): State<AppState>,
    Path(queue_name): Path<String>,
    Json(payload): Json<InsertMessage>
) -> &'static str {
    let mut queues = state.queues.lock().unwrap();
    let q = queues.queue.entry(queue_name).or_insert_with(VecDeque::new);
    q.push_back(payload.message);
    dbg!("Inserted queue {:?}", &queues);
    "pushed"
}

pub async fn pop_message(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> impl IntoResponse {
    let mut queues = state.queues.lock().unwrap();
    let q = queues.queue.entry(queue_name).or_insert_with(VecDeque::new);
    if let Some(data) = q.pop_front() {
        Json(data)
    } else {
        Json("[]".to_string())
    }
}

pub async fn get_queue_by_name(state: AppState, queue_name: String) -> Option<Vec<String>> {
    let queues: std::sync::MutexGuard<EasyQueue> = state.queues.lock().unwrap();
    if let Some(queue) = queues.queue.get(&queue_name) {
        return Some(Vec::from(queue.to_owned()));
    }
    None
}

pub async fn get_queue_msg_count(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> impl IntoResponse {
    if let Some(queue) = get_queue_by_name(state, queue_name).await {
        return Json(MessageCount {
            count: queue.len(),
        });
    } else {
        todo!()
    }
}

pub async fn get_all_queue_messages(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> impl IntoResponse {
    let queues: std::sync::MutexGuard<EasyQueue> = state.queues.lock().unwrap();
    dbg!("{:?}", &queues);
    if let Some(queue) = queues.queue.get(&queue_name) {
        return Json(Vec::from(queue.to_owned()));
    }

    // empty for now
    Json(Vec::<String>::new())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertMessage {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageCount {
    count: usize,
}
