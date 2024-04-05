# Where to Go Next

Now that you have an HTTP server, you can pretty much do whatever you want!
All clients can integrate with the HTTP server and thereby become connected.

The best way to integrate a client with the HTTP server is by implementing the [HTTP client](../http_client.md) storage backend.
The only client component that cannot benefit from that is the web app, because it runs in the browser.
To integrate that one with the HTTP server, see [Web App and HTTP](../web_app_http.md).

The one thing the HTTP server cannot do is live-updating GUIs.
If you want that, you should implement the WebSocket component.
