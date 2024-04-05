# HTTP Client

```admonish tip
You don't need your own HTTP server for the HTTP client to work!
There is a reference implementation of the server which your client can talk to.
```

Pretty much all of the available components can use an HTTP server to store paekli.
In order to avoid code duplication, we'll implement the `DistributionCenter` trait.
If you haven't created this trait yet, do the [Storage Backend](storage_backend.md) guide first and then come back here.

```rust
// somewhere in paekli-core
struct HttpClient;

impl DistributionCenter for HttpClient {
    // ...
}
```

## Storing paekli

The most common library for making HTTP requests is `reqwest` (nope, that's not a typo 😄).
Add it to your shared library `paekli-core` with a couple feature flags:

```sh
cargo add reqwest --features blocking,json
```

Here's a brief explanation for each feature flag.
They are appropriately mentioned in the documentation ([docs.rs/reqwest](https://docs.rs/reqwest)), so you shouldn't have any issues picking the right ones if you use `reqwest` on your own.
- `blocking`\
  `reqwest` is `async` by default, which is a language feature that we haven't discussed.
  In order to use the simpler _blocking_ API, we need to enable that flag.
- `json`\
  This one is intuitive, it enables JSON (de-)serialization via `serde_json`.

Now, to create an HTTP request for storing a paekli, we're gonna need to create a _client_ with `reqwest` and call its `post` method:

```rust
let client = reqwest::blocking::Client::new();
client
    .post("https://paekli.buenzli.dev/paekli")
    .json(todo!())
    .send()
    .unwrap();
```

```admonish tip title="Using your own HTTP server" collapsible=true
If you want to send the request to your own server, just replace the URL string with something like `http://localhost:4200/paekli` (make sure to get the right port).
If you want to get real fancy, try making the URL configurable in the `HttpClient`!
That way, users of this storage backend could use it to connect to any HTTP server they like.
```

Notice that I left out the JSON body.
You have two options here:

1. If you're using your own server, you should extract the types like `SendRequest` and `ReceiveResponse` into the shared library `paekli-core` so you can use them in the client as well.
  That way you can ensure that client and server always agree on the structure of the data that's being sent.
1. If you're using the reference server, read its API documentation at [paekli.buenzli.dev](https://paekli.buenzli.dev) to find out what structure of data it's expecting.

Either way, you should have some kind of struct which implements `serde::Serialize` which you can then pass into the `.json()` call.

## Retrieving paekli

To retrieve paekli, we need the `delete` method instead of `post`.
Obviously we also have to send a different request body, depending on what the server expects.

You may have discarded the response from the `store` operation, or you may have read it for error handling / reporting purposes.
Now, for the `retrieve` operation, we have to read the response to get the paekli content.
The response is what you get from the `.send().unwrap()`.
(you are encouraged to do better error handling than me 🙈)

Here's how to get the JSON response:

```rust
let data: ReceiveResponse = resp.json().unwrap();
```

Again, `ReceiveResponse` just has to be some type that implements `serde::Deserialize` and actually matches what we expect the server to respond.

## Extending the constructor function

The implementation of the `DistributionCenter` trait should be complete at this point, but we still need a way for `paekli-core` users to get their hands on an instance of `HttpClient`.
You will likely have to add some parameter to the constructor function, so users can select which storage backend they want.
As the number of backends grows, maybe an `enum` makes sense?
If some backends can even be configured further (e.g. server URL), such configuration data can be stored alongside the enum variant.
I'll leave the details up to you.

```admonish success
That should be it!

Now you can extend your clients to make use of the new backend.
```
