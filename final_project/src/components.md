# Components

This page contains some general information about components.
Notably, it should help you pick your first one to implement.
The full list of available components is in the sidebar.

Pretty much all components can stand on their own so you **can** choose any one you like.
Are you super curious about using Rust to build [Python extension modules](components/py_module.md)?
Go for it, don't let me stop you!

_However..._

Towards the bottom of the list, the components get into the territory of servers and storage backends.
Those are probably less suitable as a first component without any clients.
The HTTP server is a gray zone, you could use that on its own with `curl`.

Unless you have a strong desire to do something else, I recommend to **start with the [CLI](components/cli.md)**.
It is the easiest component to implement and its guide is the most detailed.
The other guides are still self-contained, but they provide fewer explanations of steps that are identical or similar for all components.

Once you've completed a component, several possible integrations will be suggested to you, depending on the other components that you already have.
Naturally, you need at least two components before you can do any integration.
