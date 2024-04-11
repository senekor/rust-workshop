# Where to Go Next

The web app is a bit more limited in the set of integrations it supports, because it runs in the browser.
For example, a web app cannot talk to a Unix socket or SQLite database.

However, there are fantastic integrations with technologies native to the browser.

Firstly, we have of course the HTTP server.
This is the place to start if you want to connect your web app to your other components.

Secondly, there is WebSocket.
This technology enables a web app and a server to have an ongoing connection where both sides can initiate messages.
HTTP already enables client-to-server initiated messages, so the really interesting part is is server-to-client initiated messages via websocket.
In simple terms: **push notifications**!
That means with websocket, your web app can instantaneously update its GUI when your server receives a new paekli from a completely unrelated component.

The way to get there involves a couple steps though.
You probably want to have an HTTP server already, otherwise stuff just gets more complicated (even though it's possible).
Then you'll need to implement the websocket server, shared message types in the [shared library](../shared_lib.md) and then integrate an ongoing websocket connection into your web app.
For the exciting Aha!-moment, you'll also need another component to be integrated with the HTTP server, for example the CLI.

You get the point, it's a decent chunk of work.
Don't worry though, I'll be there to help at every step!
Just be prepared to spend some time on this if you choose to accept the challenge.
