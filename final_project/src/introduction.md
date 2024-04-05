# The Final Project

```admonish info title="State of this book" collapsible=true
This book was created in early 2024 as the final part of an in-person Rust workshop.
If you are reading it much later or out-of-context, your experience may not be optimal.

I hope to keep this book updated and make it more generally useful for people learning Rust in the future.
```

You now have all the individual skills necessary to create useful software.
You can write Rust code, use libraries, write tests and **continuously deliver** your product.
Now is the time to put everything together and ship a fully-featured application!

## The Topic

We humans learn best when we're having fun, so the application will be themed as a simulation of a postal service.
In practice, that's just a messaging application.
We choose this topic, because its feature set is very flexible and can adapt to what and how much **you** want to do.

Our postal service could be sending "packages" around, but that word has another meaning in the software world.
So, we'll be using the less ambiguous and more fun word ✨ **paekli** ✨ to refer to deliverables in our business domain.

## Choose your own components

This book is structured around self-contained guides for what I call _components_.
You can choose to implement almost any combination of them in almost any order.
In its simplest form, the application can be a single component, e.g. a CLI app, without any integrations with other components.
If you choose to go the extra mile, the application can grow into a diverse set of networked clients and servers.

For example, a python script might call into a Rust library and send a paekli to an http server which live-updates a wasm-based browser app over websocket!
So cool! 🤩

As you can see in the sidebar, the components are loosely categrized into **clients**, **servers** and **storage backends**.
- Clients can be stand-alone and may use servers and storage backends.
- Servers can be stand-alone, but are probably difficult to use without a client. They may also use storage backends.
- Storage backends can't be stand-alone, as their only purpose is to be used by clients and servers.

Unless you have a strong desire to do something else, I recommend to **start with the [CLI](cli.md)**.
It is the easiest component to implement and its guide is the most detailed.
The other guides are still self-contained, but they provide fewer explanations of steps that are identical or similar for all components.
