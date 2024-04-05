use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use axum::{
    http::StatusCode,
    routing::{delete, post},
    Json,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct Inbox {
    regular: VecDeque<String>,
    express: VecDeque<String>,
}

static PAEKLI_STORE: Lazy<Mutex<HashMap<String, Inbox>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

#[derive(Deserialize)]
struct SendRequest {
    content: String,
    receiver: Option<String>,
    #[serde(default)]
    express: bool,
}

/// The purpose of this "default" user is to allow interoperability with
/// client components which haven't yet implemented the additional feature
/// of individual receivers.
static ANON: &str = "anon_anyone_can_send_and_receive";

fn get_anon() -> String {
    ANON.into()
}

#[axum::debug_handler]
async fn send_paekli(Json(request): Json<SendRequest>) {
    let mut guard = PAEKLI_STORE.lock().unwrap();
    let inbox = guard
        .entry(request.receiver.unwrap_or_else(get_anon))
        .or_default();
    if request.express {
        inbox.express.push_back(request.content);
        if inbox.express.len() > 20 {
            // prevent DoS attack at the cost of reliability
            inbox.express.drain(..5);
        }
    } else {
        inbox.regular.push_back(request.content);
        if inbox.regular.len() > 20 {
            // prevent DoS attack at the cost of reliability
            inbox.regular.drain(..5);
        }
    }
    if guard.len() > 100 {
        // prevent DoS attack at the cost of reliability
        guard.drain();
    }
}

#[derive(Debug, Deserialize)]
struct ReceiveRequest {
    receiver: String,
}

#[derive(Serialize)]
struct ReceiveResponse {
    content: String,
}

#[axum::debug_handler]
async fn receive_paekli(
    request: Option<Json<ReceiveRequest>>,
) -> Result<Json<ReceiveResponse>, StatusCode> {
    let mut guard = PAEKLI_STORE.lock().unwrap();
    let inbox = guard
        .entry(request.map(|Json(r)| r.receiver).unwrap_or_else(get_anon))
        .or_default();

    if let Some(content) = inbox.express.pop_front() {
        return Ok(Json(ReceiveResponse { content }));
    }
    if let Some(content) = inbox.regular.pop_front() {
        return Ok(Json(ReceiveResponse { content }));
    }
    Err(StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    let router = axum::Router::new()
        .route("/", post(send_paekli))
        .route("/", delete(receive_paekli));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
