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

#[derive(Deserialize, ToSchema)]
struct SendRequest {
    #[schema(example = "A pair of socks")]
    content: String,
    #[schema(example = "Sophia")]
    receiver: Option<String>,
    #[serde(default)]
    express: bool,
}

/// The purpose of this anonymous user is to allow interoperability with
/// client components which haven't yet implemented the additional feature
/// of individual receivers.
static ANON: &str = "anon_anyone_can_send_and_receive";

fn get_anon() -> String {
    ANON.into()
}

/// Send a paekli
///
/// An individual `receiver` may optionally be specified. If no receiver is
/// specified, the paekli goes to a "shared inbox" where anyone who doesn't
/// identify themself can receive it.
///
/// Paekli with `express` delivery will be received before the other paekli.
///
#[utoipa::path(
    post,
    path = "/paekli",
    request_body = SendRequest,
    responses(
        (status = 200, description = "Paekli sent successfully")
    )
)]
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

#[derive(Debug, Deserialize, ToSchema)]
struct ReceiveRequest {
    #[schema(example = "Sophia")]
    receiver: String,
}

#[derive(Serialize, ToSchema)]
struct ReceiveResponse {
    content: String,
}

/// Receive a paekli
///
/// An optional JSON body with a `receiver` may be used for identification, in
/// order to receive paekli intended for oneself. Without identification, a
/// paekli from a "shared inbox" can be received.
///
#[utoipa::path(
    delete,
    path = "/paekli",
    request_body = Option<ReceiveRequest>,
    responses(
        (status = 200, description = "Paekli received successfully", body = ReceiveResponse),
        (status = 404, description = "No paekli for you 😢"),
    )
)]
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
        .route("/paekli", post(send_paekli))
        .route("/paekli", delete(receive_paekli))
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4200").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

use utoipa::{OpenApi, ToSchema};
use utoipa_rapidoc::RapiDoc;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "\
### A reference implementation of the Rust workshop HTTP component.

## ⚠️ WARNING ⚠️

The reference implementation has limited in-memory storage to prevent excessive server resource usage.
If there are too many different receivers or outstanding paekli, if will delete stuff indiscriminately.
**Reliability is not guaranteed**.",
        version = "",
    ),
    servers(
        (url = "https://paekli.buenzli.dev"),
    ),
    paths(send_paekli, receive_paekli),
    components(schemas(SendRequest, ReceiveRequest, ReceiveResponse))
)]
struct ApiDoc;