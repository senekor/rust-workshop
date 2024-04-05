# The Hello World of HTTP-Servers

## Initializing a new package

Just like for the CLI, we'll work with a new binary crate `paekli-http`.

```sh
cd rust-workshop/final_project
cargo new paekli-cli
```

## Axum

There are many great choices for web-server libraries in Rust.
[blessed.rs](https://blessed.rs/crates#section-networking-subsection-http-foundations) lists a few of them.
But I'm the one writing this book, so I get to decide.
[Axum](https://docs.rs/axum/latest/axum/) is simple, modular, performant, well-maintained and popular.
You can't go wrong with it!

```sh
cargo add axum --features macros
```

We don't actually need the feature flag `macros` for any functionality, but it has a handy tool to improve error messages in case we make a mistake.

```admonish info title="async"
Axum - as most other web-server libraries in Rust - makes use of a language feature called "asynchronous IO" or simply `async`.
Some other languages have similar features, including C#, JavaScript and Python, so you might be familiar with it.

We will _completely ignore_ that stuff in this guide, because there's quite a bit to learn about it and it really only matters for performance.

You will however see some stuff that might seem weird if you don't know about `asnyc`.
The guide will point it out as well and remind you not to worry about it.

If you want to get serious about writing web-servers in Rust, it's definitely a good idea to learn about `async` on your own.
```

## Tokio

Every Rust-program that makes use of `async` must have a so-called _async runtime_.
This will be [tokio](https://tokio.rs/) in our case, it's pretty much the "default" one in the ecosystem.
Tokio has many feature-flags, let's enable all of them to not have to worry about it:

```sh
cargo add tokio --features full
```

We will let the tokio runtime take over our main function, so let's add this goofy-looking setup:

```rust
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
```

Now, if you `cargo run` your project, it should still print "Hello, world!".

## Spinning up a useless HTTP-server

We're not quite done with the boilerplte yet:

```rust
async fn main() {
    let router = axum::Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
```

Confused about the `.await`?
Don't worry about it, it's `async` stuff 🤫

Here we are creating a `Router` from the `axum` library.
The router is responsible for deciding how to handle incoming HTTP requests.
Right now, it doesn't do anything.

After that, we create a TCP-listener with the `tokio` library and bind it to the port `3000`.

Lastly, we use the function `axum::serve` to make our previously created router respond to incoming requestion on that TCP-listener.

If you `cargo run` this, there won't be any output in the terminal.
However, you can already send requests and receive responses:

```
> curl --verbose 0.0.0.0:3000
* processing: 0.0.0.0:3000
*   Trying 0.0.0.0:3000...
* Connected to 0.0.0.0 (127.0.0.1) port 3000
> GET / HTTP/1.1
> Host: 0.0.0.0:3000
> User-Agent: curl/8.2.1
> Accept: */*
>
< HTTP/1.1 404 Not Found
< content-length: 0
< date: Fri, 08 Mar 2024 07:13:00 GMT
<
* Connection #0 to host 0.0.0.0 left intact
```

This is using `curl` to send an empty HTTP-request to our server.
Among all this HTTP-gobbledygook, the most interesting piece is `404 Not Found`.
I'm sure you've already seen this response in the browser!

So our server is telling us _it didn't find the thing we were asking for_, which is a reasonable default chosen by the `axum` library.

## Handling our first request

Let's write a simple function that will be responsible for handling a request:

```rust
async fn hello_world() -> &'static str {
    "hello world"
}
```

All HTTP-handlers need the `async` keyword before `fn`.
Don't worry about it 🙂

Now we can tell the _router_ to let some requests be handled by this function:

```rust
use axum::routing::get;

async fn main() {
    let router = axum::Router::new().route("/", get(hello_world));
}
```

The router needs two pieces of information to decide which handler is responsible for an incoming request:
- `"/"` is the so-called **path**.
  It corresponds to the part of a URL after the _domain_.
  For example, if you go to [docs.rs/axum/latest/axum/struct.Router.html](https://docs.rs/axum/latest/axum/struct.Router.html) in your browser, the request you're sending has the path `/axum/latest/axum/struct.Router.html`.
- `get(hello_world)` tells axum that only requests with the method `GET` should be handled.
  The **method** is a part of the HTTP-protocol and `GET` is the default one.
  We'll see more methods later on.

We will learn about bits and pieces of the HTTP-protocol as we need them.

If we take a second look at the output of the `curl` command from above, we might notice this line:

```
> GET / HTTP/1.1
```

This is saying exactly that `curl` sent a request with the method `GET` and the path `/`.
That means our request should now be handled!
Let's try again, (remember to restart the server with `cargo run`):

```
> curl 0.0.0.0:3000
hello world
```

Hurray!
You should also see the greeting when you navigate to [http://0.0.0.0:3000](http://0.0.0.0:3000) in the browser.

## Shipping the first version

Let's again make sure we can release our software efficiently.
Remeber that you should use a version number that is higher than the last one you used for any other component.
Otherwise, our distribution-tool `cargo-dist` might get confused.

Assuming the last highest version used was `0.1.9`:

```toml
# paekli-http/Cargo.toml
version = "0.1.10"
```

With those changes: git commit, push, tag and push the tag!

```admonish check title="Release"
You've now shipped a functioning HTTP-server ready to download for your users.
You're awesome! 😎
```
