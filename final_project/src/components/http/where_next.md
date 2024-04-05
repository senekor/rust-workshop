# Where to Go Next

Now that you have an HTTP server, you can pretty much do whatever you want!
All clients can integrate with the HTTP server and thereby become connected.

If you don't have one yet, it's probably a good idea to create at least one storage backend.
It will allow you to write clients that are independent of a server without risky code duplication.

The one thing the HTTP server cannot to is live-updating GUIs.
If you want that, you should implement the WebSocket component.
