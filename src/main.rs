#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{ extract::State, http::StatusCode, routing::{ get, post }, Json, Router };
use serde::{ Deserialize, Serialize };

#[derive(Clone)]
struct AppState {
    queues: Arc<Mutex<EphemQs>>,
}

#[derive(Clone, Debug)]
struct EphemQs {
    queue: HashMap<String, VecDeque<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InsertMessage {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetQueueRequest {
    queue_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageCount {
    count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    message: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let empty_map: HashMap<String, VecDeque<String>> = HashMap::new();
    let state = AppState {
        queues: Arc::new(Mutex::new(EphemQs { queue: empty_map })),
    };
    let app = Router::new()
        .route("/health", get(health))
        .route("/insert_message/:queue_name", post(insert_message))
        .route("/get_message/:queue_name", get(get_all_queue_messages))
        .route("/pop_message/:queue_name", post(pop_message))
        .route("/message_count/:queue_name", get(get_queue_msg_count))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn insert_message(
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

async fn pop_message(
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

async fn get_queue_by_name(state: AppState, queue_name: String) -> Option<Vec<String>> {
    let queues: std::sync::MutexGuard<EphemQs> = state.queues.lock().unwrap();
    if let Some(queue) = queues.queue.get(&queue_name) {
        return Some(Vec::from(queue.to_owned()));
    }
    None
}

async fn get_queue_msg_count(
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

async fn get_all_queue_messages(
    State(state): State<AppState>,
    Path(queue_name): Path<String>
) -> impl IntoResponse {
    let queues: std::sync::MutexGuard<EphemQs> = state.queues.lock().unwrap();
    dbg!("{:?}", &queues);
    if let Some(queue) = queues.queue.get(&queue_name) {
        return Json(Vec::from(queue.to_owned()));
    }

    // empty for now
    Json(Vec::<String>::new())
}
// basic handler that responds with a static string
async fn health() -> &'static str {
    "Hello, World!"
}
