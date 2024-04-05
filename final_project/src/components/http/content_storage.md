# Content and Storage

The send- and receive-handler are just dummies at this point, let's change that.

## Sending paekli with content

These days, it is common to use JSON to transmit information via HTTP.
We have already seen how the libraries `serde` and `serde_json` make JSON handling a walk in the park.


The content of a paekli will be another CLI argument.
Since we only expect content when _sending_ a paekli, we will add the argument to that subcommand.
Specifying the content of a paekli you're receiving doesn't make sense.
When adding `serde`, don't forget the `derive` feature.

Let's define the structure of the input we are expecting to our send handler, it's simple enough:

```rust
#[derive(Deserialize)]
struct SendRequest {
    content: String,
}
```

Now we're about to see the fun part of `axum`.
We can simply add a `Json<SendRequest>` to the parameters of our handler.
The `Json` type is from the `axum` library and instructs it to take the body of an incoming HTTP-request, deserialize it into the specified type (`SendRequest`) and pass it into the handler function.

```rust
async fn send_paekli(Json(request): Json<SendRequest>) {
    println!("sending: {}", request.content);
}
```

This is very cool and avoids boilerplate, but it's not without its downsides.
The types and order of parameters and the return type of your handler function have great significance for the behavior of your server.
That being said, the `#[axum::debug_handler]` should go a long way in helping with that.

Let's explore how our changes affected the behavior of our send-handler:

```
> curl --request POST 0.0.0.0:3000
Expected request with `Content-Type: application/json`
```

If we don't send a body with our request, we will be told that some JSON was expected.
Not too bad!
Just by adding a typed parameter to our function, `axum` generates meaningful error messages for us.
Let's lie to the server in the request by saying that we're sending JSON, without actually doing it:

```
> curl --request POST \
    --header 'Content-Type: application/json' \
    0.0.0.0:3000
Failed to parse the request body as JSON: EOF while parsing a value at line 1 column 0
```

We're being told the JSON we sent was invalid, that's also reasonable.
What about valid JSON that doesn't contain the required `content: String`?

```
> curl --request POST \
    --header 'Content-Type: application/json' \
    --data '{ "some_other_stuff": 1234 }' \
    0.0.0.0:3000
Failed to deserialize the JSON body into the target type: missing field `content` at line 1 column 28
```

Awesome.
With very little code, we're getting perfect validation and good errors every step along the way.
For completeness' sake, here's a correct example:

```
❯ curl --request POST \
    --header 'Content-Type: application/json' \
    --data '{ "content": "Legos" }' \
    0.0.0.0:3000
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.
* throws your paekli directly in the trash *
```

The `curl` commands are getting quite verbose at this point.
You might want to keep a file or even a script with a bunch of them.
The HTTP-server component is not primarily intended for direct user interaction, but rather to connect other, local components with each other.
However, that will be part of the integrations.

## Storing paekli for delivery

We could do the same as with the CLI and store the paekli in the file system.
But at that point, we would be duplicating the code interacting with the file system.
If we made a mistake or changed something in one place but not the other, it could lead to data corruption.
The solution to this is a shared storage backend, which is a component you can implement later.
For now, we're intentionally going to implement a _terrible_ way to store our data:

```rust
static PAEKLI_STORAGE: Mutex<Option<String>> = Mutex::new(None);
```

_Global mutable state_.
Are you as disgusted as I am?
Great, this will give you motivation to implement the shared storage backend component later 😉
Here is how you can store an incoming paekli:

```rust
let mut guard = PAEKLI_STORE.lock().unwrap();
*guard = Some(request.content);
```

We're not handing the error-case where a paekli exists already for now.

### Delivering paekli

Again, I'll leave the delivery of the paekli up to you.

If you are interested in doing idiomatic error-handling both in terms of Rust and web standards, here's the type your handler should probably return:

`-> Result<Json<ReceiveResponse>, StatusCode>`

In the success-case, a JSON-formatted response with whatever information the custom type `ReceiveResponse` holds.
In the error-case, an HTTP status code.
`404 Not Found` seems like a reasonable choice, in case there is no paekli.

```admonish check title="Release"
That was the MVP! A release is in order.
```
