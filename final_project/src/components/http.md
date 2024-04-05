# HTTP Server

HTTP servers are lots of fun!
The8 are simple and versatile, allowing you to make your application available anywhere with an internet connection.

This guide does **not** assume any prior knowledge about writing web servers.
So don't be scared if you've never done this before!
If you're already a battle-hardened web dev, you can simply skip over the exlanations of the basics.

What we're going to build here is sometimes also referred to as a REST-API.
We're not gonna be too strict about that term though, we'll just stick to the basic conventions of web development.

Here's an example of what the product might look like:

```
> curl --header 'Content-Type: application/json' \
       --data '{ "content": "strawberries" }' \
       https://paekli.buenzli.dev/paekli

> curl --request DELETE https://paekli.buenzli.dev/paekli
{"content":"strawberries"}
```

The curl commands are a little verbose, but this is not the primary intended use case.
The point is that the sender and the receiver could be completely different devices and clients on the network!

This guide assumes that you already created [the CLI component](components/cli.md).
You can totally follow it if you haven't, but there will be fewer explanations of the basic steps.

Let's get started!
