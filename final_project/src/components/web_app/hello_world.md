# Hello World

Fair warning: The web app has a bit more setup than the other components.
Don't worry though, we'll manage just fine together!

## Installing dependencies

When you install the Rust toolchain, it only supports compiling to your native platform by default.
To add support to compile to WebAssembly, let's add the necessary target:

```sh
rustup target add wasm32-unknown-unknown
```

We're gonna use a _bundler_ called [trunk](https://trunkrs.dev/) to help us manage all the boilerplate around a web app:

```sh
cargo install --locked trunk
```

Let's initialize a new package for the web app:

```sh
cd final_project
cargo new paekli-web
```

Add a couple libraries we'll need:

```sh
cd paekli-web
cargo add gloo
cargo add leptos --features csr
cargo add console_error_panic_hook
```

Trunk puts its output into a different directory than cargo, you probably want to git-ignore that:

```sh
# still inside paekli-web/
echo dist > .gitignore
```

## Dummy `index.html`

A normal, plain website might be nothing more than an HTML file.
That's exactly how our web app is going to _start_ as: a plain HTML file without any content.
Its only purpose is to load the wasm code that generates the interactive web app.
Hopefully that makes sense, but if it doesn't, don't worry.
I'm just trying to explain why we need the following boilerplate.

Add an `index.html` _next to your Cargo.toml_.
The location is important.
Add the following content:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Hello Rust Workshop!</title>
  </head>

  <body></body>
</html>
```

That's already enough to get the (empty) web app off the ground.
Let's start the development server of our bundler `trunk`:

```sh
trunk serve
```

It might take a minute to compile the first time, but then it should display a URL where you can see your web app.
It's probably [localhost:8080](http://localhost:8080).

The page will be empty (because of the empty `<body></body>` tag), but the custom title should be visible in the browser tab.

## Running Rust in the browser

So far, we haven't actually written any Rust that compiles to wasm to run in the browser.
Let's change that.
How about this:

```rust
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <h1>Hello WebAssembly!</h1>
        }
    })
}
```

If you add this change to `main.rs` while you keep `trunk serve` running, your web app should automatically recompile and _reload in the browser_ once compilation is done.
Pretty decent development experience!
Can you see the result in the browser?

Let's explain a little bit what's going on here.
Remember the empty `<body></body>` from the `index.html`?
This body tag is the container of all the content on a website.
With the function `mount_to_body`, we're letting our Rust code take over the body tag and therefore the entire content of the website.
The argument passed to `mount_to_body` is a simple function (notice that `||` is in this case the start of a _closure_ without any arguments).
The function returns a `view!` macro with the content `<h1>Hello WebAssembly!</h1>`.
The `view!` macro allows us to write HTML-like syntax which will be inserted into the website.
This might look alien to you, but it's very convenient for seasoned web developers and pervasive in the JavaScript world.

## Setting up decent error messages

We're still inexperienced when it comes to web development with Rust, so we might make a mistake or two.
If our app crashes, it would be nice to get some decent error message.
Let's add the following at the top of our main function:

```rust
console_error_panic_hook::set_once();
panic!("I don't know what to do!");
```

In the browser, the "Hello WebAssembly" text should be gone.
This is because the app crashed before it was able to display the text.
If you open the browser dev tools with `F12` and click on the "Console" tab, you should see our custom error message and a line number.
That's good enough for our purposes!

Remove the `panic!` statement (but keep the panic hook) and your app should work again.

```admonish check title="~~Release~~ sorry, we're not quite there yet"
Congratulations!
You are now a fully-oxidized web developer 🥳

You might expect me to pester you about cuttting a relaeas at this point.
But setting up the release of the web app is a little more work, so it deserves its own section.
It'll be worth it though!
In contrast to the other components, your users _won't even have to install your web app_.
We will be deploying to GitHub Pages, freely accessible to anyone who can click a link!
```
