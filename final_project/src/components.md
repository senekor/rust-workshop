# Components

This is some general information about components.
Notably, you'll find help to pick your next one to implement.
The full list of available components is in the sidebar.

Pretty much all components can stand on their own so you **can** choose any one you like.
Are you super curious about using Rust to build [Python extension modules](components/py_module.md)?
Go for it, don't let me stop you!

Towards the bottom of the list, the components get into the territory of servers and storage backends.
Those are probably less suitable as a first component without any clients.
The HTTP server is a gray zone, you could use that on its own with `curl`.

Once you've completed a component, several possible integrations will be suggested to you, depending on the other components that you already have.
Naturally, you need at least two components before you can do any integration.

Unless you have a strong desire to do something else, I recommend to **start with the [CLI](components/cli.md)**.

If you already have a CLI and can't decide what to do next, I would suggest the [HTTP server](components/http.md) as the second component and the [web app](components/web_app.md) as the third.
