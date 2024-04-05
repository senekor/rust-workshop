# Notifications

Let's say we want to send a notifications to all of our subscribers once a new paekli was sent, i.e. it is ready to be received.

Now we've got a bit of a problem.
The two handler functions `send_paekli` and `subscribe_to_notifications` are completely separate!
They cannot talk to each other at all.

Thankfully, `axum` has a solution for this problem.
We can pass some "shared state" to all handlers.
In this case, the state is just going to be a channel through which the handlers can communicate with each other.
The channel itself is provided by the tokio runtime.

```rust
let (notification_sender, _) = tokio::sync::broadcast::channel(16);
```

The argument `16` is the channel capacity, it shouldn't matter for our purpose.
The second return value is a _receiver_ from that channel, but we don't care about it for now.
We can always create new receivers by subscribing on the sender again.
Let's pass this channel as shared state to all handler functions:

```rust
axum::Router::new()
    // ...
    .route()
    .with_state(notification_sender)
```

There will likely be some compile error now because a type cannot be inferred.
This should be fixed shortly.
Let's use this sender in the `send_paekli` handler:

```rust
async fn send_paekli(
    State(sender): State<Sender<()>>,
    // other params...
) {
    // ...
    sender.send(()).unwrap();
}
```

The necessary imports are `axum::extract::State` and `tokio::sync::broadcast::Sender`.
We specify the type of notification being sent as the empty tuple with `Sender<()>`.
This can be changed later, if we actually want to send information with the notification.

The semantics are simple, the `State` wrapper around our sender tells `axum` to pass in the shared state we gave to the router earlier.
Then we send an empty tuple over the channel to indicate a paekli was sent.

Now we can listen on this channel in the `subscribe_to_notifications` handler:

```rust
async fn subscribe_to_notifications(
    ws: WebSocketUpgrade,
    State(sender): State<Sender<()>>,
) -> axum::response::Response {
    // ...
}
```

Within the function body, you'll have to make three modifications:

- get a new receiver by calling `sender.subscribe()`
- loop over incoming notifications with `receiver.recv().await` (just send any string over websocket in the loop body)
- `move` the channel into the closure: `ws.on_upgrade(|mut socket| async move {` (otherwise you'll get a lifetime error)

```admonish success
At this point, you should be able to listen on mutliple websocket connections and get a notification every time a paekli is sent.
```

This is already enough in terms of the MVP.
Feel free to work on an integration next, like [Web App and WebSocket](../web_app_websocket.md).
That being said, there is one more section about handling individual recipients.
