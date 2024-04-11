# Content and Storage

Accepting user input in HTML is simple enough:

```html
<input placeholder="paekli content"></input>
```

You should already be able to use this text field in the browser.
However, it's not obvious how our Rust code can access the text our user types in.

## Storing values in Rust

There are multiple ways to do this, but we'll go with the simplest route.
We'll create a variable in our Rust code where the text will be stored and instruct the `<input>` element to update the value of that variable when the user types in a character.
A regular `let mut` variable doesn't quite cut it though, we'll need a special type from the `leptos` library called a `Signal`.
But the principle is exactly the same!

```rust
// reading and writing of the signal are separated into two variables
let (get_content, set_content) = create_signal(String::default());

view! {
    <input
        placeholder="paekli content"
        prop:value=move || get_content.get()
        on:input=move |e| set_content.set(event_target_value(&e))
    ></input>
}
```

Alright, I'll explain this in a second.
But first, you're probably getting a compiler error at this point.
It says something like:

```admonish fail title="compiler error"
bla bla **closure may outlive the current function** bla bla **use the `move` keyword**.
```

Okay, that sounds kinda difficult.
It's related to Rust's lifetime system.
The fundamental problem here is that user interfaces are long-lived programs where is becomes harder for the borrow checker to make sure you're not using some value after it isn't valid anymore.
Luckily, the developers of the `leptos` library have a perfect solution for this problem and the Rust compiler _even tells us the correct thing to do_.
We have to put the `move` keyword in front of the closure passed to `mount_to_body`:

```rust
mount_to_body(move || {
    // ...
}
```

The only downside of this solution is that it's not obvious at all what's going on.
To truly understand it, one needs decent experience with the borrow checker and a good understanding of how the `leptos` library works internally.
Needless to say, this is beyond the scope of this guide.
So for now, I can only tell you to "trust me bro" and `move` all the closures!

Alright, I still owe you an explanation of the code snippet above.
First we create a "signal", `leptos` special variable type that's separated into a getter and setter function.
The reason we need a signal instead of a regular variable is because leptos automatically updates all the places in the UI where the signal is used when it changes.
That's not something we would be able to do with regular variables.
We control the current content of the input field with the `prop:value` attribute.
It's value is a function that returns the current content of the signal.
That way, the input field and the signal are always in sync.
We also have a new event handler for the `on:input` event.
This function will be executed every time the user changes the content of the input field.
The function accepts an event `e` and sets the value of the signal to the `event_target_value` of `e`, which is precisely the new content of the input field.
"Event", "target" and "value" are all terms with specific meaning in the world of browsers and web development, so don't worry about it too much if it seems alien to you.

You can read more about the nuances of handling user input in the [Leptos documentation](https://book.leptos.dev/view/05_forms.html).

You might find `get_content.get()` and `set_content.set()` a little verbose.
There is a nicer way to write this, but it requires the nightly compiler for now, so I chose to avoid it.

Just one last thing, let's display the paekli content in the alert when sending it, to confirm everything works as expected:

```rust
<button
    on:click=move |_| alert(&format!("paekli with {} was sent!", get_content.get()))
>
    Send
</button>
```

Now enter some text, click send and observe that the correct value is displayed.

Phew.
I don't know about you, but I'm exhausted now.
Feel free to take a break, web development is hard! 😮‍💨

## Paekli storage

We now have a signal to store the input of our users.
But we should probably clear that when a paekli is sent and commit the content to a different signal.
Otherwise we cannot distinguish between paekli content that was actually sent and the user just derping around in the text field.

I think you can manage on own.
I believe in you!! 💪

## Receiving paekli

I'll leave this up to you as well.
You should be fine with all the things we've seen already.
Make sure that a paekli cannot be received twice!

```admonish check title="Release"
I gotta say, it's not as exiting to tell you to release your software when it's literrally just pushing to the main branch 🥲
```
