# Web App and WebSocket

If you don't have the [WebSocket](websocket.md) component yet, you can still integrate with the reference implementation.
This guide assumes you have the feature for individual recipients implemented, but it should work without it with a few adjustments.

Let's first think about how we want to handle the websocket connection on the client side.
When users first open the app, we don't want any websocket connection to be opened.
Users should then be able to select a recipient (presumably themselves) and open a websocket connection to receive the notifications for that recipient.
Users should also be able to stop receiving notifications, i.e. close the websocket connection.

The most idiomatic way to achieve such a life cycle for resources in our UI library `leptos` is to tie it to the life cycle of a UI component.
To open a websocket connection, we render the component.
To close the connection, we stop rendering the component.
Sounds complicated and abstract for now, but we'll get there one step at a time.

## Creating a custom component

Until now, we only had a `main` function and all our declarative UI was in a single `view!` macro call.
That macro contained standard HTML elements.
`leptos` allows us to define our own components and use them as if they were HTML elements:

```rust
#[component]
fn NotificationListener() -> impl IntoView {
    view! {
        "Hello from a custom component!"
    }
}

fn main() {
    // ...

    mount_to_body(move || {
        view! {
            // ...
            <NotificationListener/>
        }
    })
}
```

If you have `trunk serve` running, you should now see the text in the custom component in your app.

## Rendering conditionally

Thinking about our final goal again, we need to be able to render this component conditionally, based on whether a websocket connection should be open or not at any given moment.

Let's start simple and use a _signal_ to store a boolean.
A button can then toggle the signal and cause our component to be rendered or not.

```rust
let (get_should_render, set_should_render) = create_signal(false);
let toggle_should_render = move |_| set_should_render.update(|prev| *prev = !*prev);
view! {
    <button on:click=toggle_should_render>
        toggle rendering
    </button>
}
```

Great, now let's render our custom component only if the boolean signal is `true`.
We can totally use `if` expressions for this, but it actually gets quite ugly to nest such expressions inside a `view!` macro.
It's also easy to accidentally make the UI non-reactive that way.
(Nested expressions have to be _wrapped in closures_ to stay reactive.)
`leptos` has a nice `Show` component to make conditional rendering readable and robust.

```rust
view! {
    <Show when=move || get_should_render.get()>
        <NotificationListener/>
    </Show>
}
```

The text of the custom component should now appear and disappear when you click the button.

## Using a websocket connection

Now we want to associate an open websocket connection to our custom component.
Implementing this completely ourselves would be quite complicated.
Luckily, the library `leptos-use` already did it for us!

```sh
cargo add leptos-use
```

We can simply call the function `leptos_use::use_websocket` with the URL.
The return type is `UseWebsocketReturn`, which is a struct that we can _destructure_ to only accept what we actually need and discard anything else.
This struct contains a bunch of things to control the websocket connection at a low level, but we only care about being able to read incoming messages.

```rust
let url = "ws://localhost:4200/notifications/alice";
let UseWebsocketReturn { message, .. } = leptos_use::use_websocket(url);
```

What we now need to do is _create a side effect_ that triggers every time we receive a notification.
The concept of a side effect is related to `leptos`' reactivity system, which we discuss in detail here.

```rust
create_effect(move |_| {
    if let Some(message) = message.get() {
        gloo::dialogs::alert(&message);
    }
});
```

Suffice it to say, the closure passed to `create_effect` will rerun every time there is a new message coming in over the websocket connection.

You should now be able to observe that an alert is triggered in the browser if a paekli is sent to alice.

## Making the recipient configurable

This I will leave mostly up to you, except for one feature of `leptos` that we haven't seen so far.
You can pass arguments to custom components, also referred to as "attributes" from the HTML perspective.
When declaring the component, it looks like a regular parameter.
When you use the component, it works with the `key=value` syntax like a normal HTML element.

```rust
#[component]
fn NotificationListener(recipient: String) -> impl IntoView {}

view! {
    <NotificationListener recipient=String::from("alice") />
}
```

```admonish success
That's a live-updating GUI!
You made it! 🥳
```

Feel free to improve the UX in any way you like.
What if instead of an alert, you displayed a counter of unreceived paekli?
