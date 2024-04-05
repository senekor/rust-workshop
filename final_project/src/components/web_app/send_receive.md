# Sending and Receiving

Let' start implementing the two basic interactions with the system.

First we'll need an HTML button the user can click on:

```rust
view! {
    <button>Send</button>
}
```

That's great, but it doesn't to anything yet.
To make the button functional, we'll attach a function to the button, such that the function will be executed when the button is clicked.
If this looks kinda magical to you, then that's because it is.
The `view!` macro is doing _a lot_ of heavy lifting here.

```rust
use gloo::dialogs::alert;

view! {
    <button
        on:click=|_| alert("paekli was sent!")
    >
        Send
    </button>
}
```

Apart from the formatting, what has changed?
We gave a new "attribute" to the button tag, although it's not quite valid HTML.
The `on:click` "attribute" is the special way to attach event handlers to DOM elements in `leptos`.
(In the JavaScript world, React has the almost identical `onClick`.)
The value of the special attribute is a function or _closure_.
It takes one parameter but ignores it.
The parameter would've been an click-event object with some metadata about what triggered the button-press.
The body of the function calls `gloo::dialogs::alert`, which is a binding to the alert API of the browser.
It should create a little pop-up window with the given text.

Check in your browser if this behaves the way you would expect!

Then, add a second button for receiving paekli.

```admonish check title="Release"
The buttons aren't very useful yet, but let's not make our users wait for any new features.
Cut a new release by pushing these changes to the main branch.

Remember, for the web app you don't even have to push a version tag to trigger a release.
The version tag is the trigger for `cargo dist`, which manages our downloadable binaries.
The GitHub Pages deployment is triggered by every push to main.
```
