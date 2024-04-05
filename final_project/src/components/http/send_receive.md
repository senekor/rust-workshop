# Sending and Receiving Paekli

Our server can now say hello, but we want it to help us send and receive paekli.
Let's stub out a handler for each type of request.

```rust
#[axum::debug_handler]
async fn send_paekli() -> &'static str {
    "\
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.
* throws your paekli directly in the trash *"
}

#[axum::debug_handler]
async fn receive_paekli() -> &'static str {
    "\
There aren't any paekli for you at the moment.
* tries to hide paekli in the trash can *"
}
```

Note the new `#[debug_handler]` attribute above the handler function.
This is what we needed `--features macros` for when we added the `axum` dependency.
It is a macro that provides better error messages in case there's something wrong with our handler function.
I recommend you add it to **every** new handler you're going to write.
I also recommend not making any mistakes in the first place, but... you know.

## Picking the right HTTP-method

We have already seen the HTTP-method `GET`, which is the default one.
It is used to ask a server for some information.
There are other methods with specific meanings, the most common ones are:
- `GET` for reading data
- `POST` for storing new data
- `PUT` for modifying existing data
- `DELETE` for deleting data

These four methods cover all the operations that are commonly performed on data.

`POST` is the method that most closely matches the operation of sending a paekli, because the server will need to store a new piece of data. `DELETE` seems appropriate for receiving, since the paekli should be deleted on the server after it has been received.

Recall that the router also needs a _path_ to register a handler, but we won't have to worry about that.
We just use the empty or "root" path.
The _path_ is conventionally used to specify a resource.
Since we only have one resource (paekli), the path doesn't matter.
Here's how we can register these new handlers with different HTTP-methods:

```rust
let router = axum::Router::new()
    .route("/", post(send_paekli))
    .route("/", delete(receive_paekli));
```

Let's try them out:

```
> curl --request POST 0.0.0.0:3000
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.
* throws your paekli directly in the trash *

> curl --request DELETE 0.0.0.0:3000
There aren't any paekli for you at the moment.
* tries to hide paekli in the trash can *
```

Great!
We can now execute different functions based on the incoming request.
