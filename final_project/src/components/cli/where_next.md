# Where to Go Next

This was probably your first component, in which case there aren't any integrations available yet.
So you probably only need to looks at the components section.
The guide for whatever you pick can be found in the sidebar.

## Components

- If the CLI was your first component, the recommended next one is the [HTTP-server](components/http.md).
  It is the simplest server component and unlocks (indirect) integration with any other client component.
- The other server components (WebSocket, gRPC) are more advanced / niche technologies, so you should only choose them if you have a particular interest in them.
- The other client components cannot integrate directly with the CLI.
  Clients need a server component as a middleman.
  If you're fine with that, feel free to implement another client next.
  The author's personal favorite is the [web app](components/web_app.md):
  It's portable, easy to deploy and doesn't require a single line of JavaScript 😎
- Another goal that might be worth working towards:
  Once you have a WebSocket-server and a GUI-client, you can make the GUI live-updating! 🤯

## Integrations

- If you have a server-style component (HTTP, WebSocket, gRPC), you can integrate with them directly.
- The CLI cannot integrate with other native clients directly, but they can be made to play nice together with shared _storage backends_.
  These are separate components that you need to implement first, before integrating the CLI with them.
- The web app cannot be integrated with the CLI at all.
  You need a server-style component (HTTP, WebSocket) between them.
