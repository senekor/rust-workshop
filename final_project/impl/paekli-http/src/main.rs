use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    routing::{delete, get, post},
    Json,
};
use once_cell::sync::Lazy;
use paekli_core::http_api::{ReceiveRequest, ReceiveResponse, SendRequest};

#[derive(Default)]
struct Inbox {
    regular: VecDeque<String>,
    express: VecDeque<String>,
}

static PAEKLI_STORE: Lazy<Mutex<HashMap<String, Inbox>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

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
#[axum::debug_handler]
async fn send_paekli(State(sender): State<Sender<String>>, Json(request): Json<SendRequest>) {
    let mut guard = PAEKLI_STORE.lock().unwrap();
    let recipient = request.receiver.unwrap_or_else(get_anon);
    let inbox = guard.entry(recipient.clone()).or_default();
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
    sender.send(recipient).unwrap();
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

/// Subscribe to notifications
///
/// Subscribe to WebSocket notifications.
///
#[utoipa::path(get, path = "/notifications/:recipient")]
#[axum::debug_handler]
async fn subscribe_to_notifications(
    ws: WebSocketUpgrade,
    Path(recipient): Path<String>,
    State(sender): State<Sender<String>>,
) -> axum::response::Response {
    ws.on_upgrade(|mut socket| async move {
        let mut receiver = sender.subscribe();
        while let Ok(recipient_2) = receiver.recv().await {
            if recipient_2 == recipient {
                socket
                    .send(axum::extract::ws::Message::Text("Hello, world!".into()))
                    .await
                    .unwrap();
            }
        }
        socket.close().await.unwrap();
    })
}

#[tokio::main]
async fn main() {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_millisecond(200)
            .burst_size(5)
            .key_extractor(GlobalKeyExtractor)
            .finish()
            .unwrap(),
    );

    let (notification_sender, _) = tokio::sync::broadcast::channel(16);

    let router = axum::Router::new()
        .route("/paekli", post(send_paekli))
        .route("/paekli", delete(receive_paekli))
        .route("/notifications/:recipient", get(subscribe_to_notifications))
        .with_state(notification_sender)
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/"))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        )
        .layer(GovernorLayer {
            config: Box::leak(governor_conf),
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4200").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

use tokio::sync::broadcast::Sender;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor, GovernorLayer,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "\
### A reference implementation of the Rust workshop HTTP component.

## ⚠️ WARNING ⚠️

The reference implementation has limited in-memory storage to prevent excessive server resource usage.
If there are too many different receivers or outstanding paekli, if will delete stuff indiscriminately.
**Reliability is not guaranteed.**

Also note there is a global rate-limit of five requests per second.
Please don't launch a DoS attack against my Raspberry Pi...

The necessary CORS headers _should_ be set on the server for you to access the API from a web app on a different domain.
If you still encounter issues, please notify me so I can reconsider my life choices.",
        version = "",
    ),
    servers(
        (url = "https://paekli.buenzli.dev"),
    ),
    paths(send_paekli, receive_paekli, subscribe_to_notifications),
    components(schemas(SendRequest, ReceiveRequest, ReceiveResponse))
)]
struct ApiDoc;
