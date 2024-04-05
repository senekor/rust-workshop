# Individual Recipients

We can now send and receive notifications anytime a paekli is sent.
However, our users probably only want to be notified if there is a paekli for a specific recipient, i.e. themselves.

Let's broadcast information about the receiver of the paekli in our `send_paekli` handler.
We'll need to change the type of our channel from `Sender<()>` to `Sender<String>`.
Then we just... `sender.send(receiver)` in the body.
You should already have access to this `receiver` if you implemented the additional feature of individual recipients for the HTTP server component.
We receive the notifications in the `subscribe_to_notifications` handler function, so we'll need to update the `Sender` type there as well to fix any compilation errors.

Now we could just pass along the name of the receiver from the internal channel to the websocket.
That would be simple and serve the purpose.
But it's a little nicer if the user can specify a recipient up front and only receive those notifications.

## The recipient as path parameter

In terms of our API, we'll allow users to subscribe to notifications on different URL _paths_.
For example, Alice can subscribe to her notifications at `/notifications/alice` and Bob can subscribe at `/notifications/bob`.

But we don't want to write a new handler function for ever possible recipient... that would be a lot.
Axum allows us to turn some segments of the path into parameters to the handler function.
To do that, we need to first declare the route with the new path:

```rust
.route("/notifications/:recipient", get(subscribe_to_notifications))
```

Notice the colon before `recipient`, it means this is a parameter instead of a literal part of the path.
Next, we can accept the parameter in our handler:

```rust
async fn subscribe_to_notifications(
    Path(recipient): Path<String>,
    // ...
) -> Response {
```

`Path` is imported from `axum::extract::Path`.
This type tells the `axum` library to extract the value of `recipient` from the URL of the request.
That only works because we declared the parameter previously in the route declaration with `"/notifications/:recipient"`.

````admonish note title="More than one path parameter" collapsible=true
We only need one path parameter here, so there's not much that can go wrong.
But what if we had multiple path parameters?
For example: `"/notifications/:recipient/:sender"` to only get notified for paekli from a specific sender?

That works too, but then we have to accept the parameters as a _tuple_ inside the `Path` wrapper from `axum`:

```rust
async fn subscribe_to_notifications(
    Path((recipient, sender)): Path<(String, String)>,
    // ...
) -> Response {
```

The names of the parameters `recipient` and `sender` don't really matter.
`axum` gives us the parameters in a tuple in the order they appear in the URL path.
````

Inside the body of `subscribe_to_notifications`, we can now send a notification on the websocket only if the recipient matches the one of our URL parameter.
The code you'll need might look a little something like this:

```rust
let Ok(channel_recipient) = receiver.recv().await
if url_path_recipient == channel_recipient {
    socket.send(/**/)
}
```

## Trying it out

With all these connections, it's becoming more tricky to test that everything works.
Here are a couple commands to get you started.
You might have to adjust some details, especially if you designed your HTTP API a little differently.

```sh
# Terminal 1: run HTTP & WebSocket server
cd final_project/paekli-http
cargo run
```

```sh
# Terminal 2: listen to notifications for Alice
websocat ws://127.0.0.1:4200/notifications/alice
```

```sh
# Terminal 3: listen to notifications for Bob
websocat ws://127.0.0.1:4200/notifications/bob
```

```sh
# Terminal 4: send paekli

# send paekli to Alice
curl --header 'Content-Type: application/json' --data '{ "content": "shoes", "recipient": "alice" }' localhost:4200/paekli

# send paekli to Bob
curl --header 'Content-Type: application/json' --data '{ "content": "shoes", "recipient": "alice" }' localhost:4200/paekli
```

Instead of using `curl` to send paekli, you can obviously also use any of you client components if you have already integrated them with the HTTP client storage backend.

```admonish success
Phew, that was a little tedious!
Hopefully everything worked out as expected.
🤞
```

Obviously we don't want to handle websocket connections as manually as we did just now.
The real fun starts when we integrate that with our GUI clients!
I recommend you do that next, otherwise all that work was for nothing.
