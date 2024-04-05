# WebSocket Server

```admonish warning title="Building on the HTTP server"
The WebSocket component can _technically_ be implemented without the HTTP server, but it's not very useful and makes everything more complicated.

Therefore, this guide assumes you have completed the [HTTP server](http.md).
```

WebSocket is a protocol that allows us to easily send messages from the server to the client.
This is in contrast to HTTP, where we are limited to requests initiated by the client.
The most important use case for websockets is a live-updating GUI.
In our case, the server can let its clients know immediately when a new paekli is available.
