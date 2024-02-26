# Servers

Under this section, you will find the servers you can implement.

Although an [HTTP-server](servers/http.md) can technically stand on its own, it makes sense to implement a client first.

The [WebSocket](servers/websocket.md) server is not a full server either, but rather an extension of the HTTP-server.
It has its own section because it's quite different from HTTP and unlocks a completely new set of client integrations.

[gRPC](servers/grpc.md) is an advanced technology for more specific use cases.
You should probably do HTTP and WebSocket first, unless you are specifically interested in gRPC.
