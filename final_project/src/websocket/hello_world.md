# The Hello World of WebSocket

The websocket stuff will be built on top of the HTTP server, so we don't need a new package this time.
We're mostly going to work in `paekli-http`.

## Axum WebSocket feature flag

Axum has built-in support for websockets, but it is gated behind a feature flag.
Add it with the following command:

```sh
cargo add axum --features ws
```

## Opening a WebSocket connection

First of all, we'll need a new route where clients can request to open a websocket connection:

```rust
axum::Router::new()
    // ...
    .route("/notifications", get(subscribe_to_notifications))
```

Next, we actually have to write that handler.
Don't worry, everything will be explained in a second.

```rust
#[axum::debug_handler]
async fn subscribe_to_notifications(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(|mut socket| async {
        socket
            .send(axum::extract::ws::Message::Text("Hello, world!".into()))
            .await
            .unwrap();
        socket.close().await.unwrap();
    })
}
```

- `WebSocketUpgrade` is a type from axum, assuming the feature `ws` is enabled.
  Axum will only call this handler with requests for websocket connections.
- In the body, we immediately pass a closure to `ws.on_upgrade()`, which is run once the connection is successfully established.
- Once we have the `socket` in the closure, we send a single message `"Hello, world!"` and close it again.
- Don't worry about `async` and `await`.

Websockets can be tricky to debug.
One option is to use [websocat](https://github.com/vi/websocat):

```sh
websocat ws://127.0.0.1:4200/notifications
```

```admonish success
This should print "Hello, world!" to the terminal.
```
