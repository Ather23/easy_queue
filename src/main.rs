#![forbid(unsafe_code)]
use tracing::{ info, Level };

mod web;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use axum::{ routing::{ get, post }, Router };
use serde::{ Deserialize, Serialize };
use web::*;

#[derive(Clone)]
struct AppState {
    stream_hashmap: Arc<Mutex<StreamingMap>>,
}

#[derive(Clone, Debug)]
struct StreamingMap {
    hashmap: HashMap<String, VecDeque<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    message: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting server...");

    let empty_map: HashMap<String, VecDeque<String>> = HashMap::new();
    let state = AppState {
        stream_hashmap: Arc::new(Mutex::new(StreamingMap { hashmap: empty_map })),
    };
    let app = Router::new()
        .route("/health", get(health))
        .route("/insert_message/:queue_name", post(insert_message))
        .route("/get_message/:queue_name", get(get_all_queue_messages))
        .route("/pop_message/:queue_name", post(pop_message))
        .route("/message_count/:queue_name", get(get_queue_msg_count))
        .with_state(state);
    if let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        axum::serve(listener, app).await.unwrap();
    } else {
        panic!("Could establish listener")
    }
}

async fn health() -> &'static str {
    "Hello, World!"
}
