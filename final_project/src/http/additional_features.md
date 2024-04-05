# Additional Features

Now you have the opportunity to bring the HTTP server to feature-parity if you have already implemented the additional features for the CLI.
However, note that it will require you to make the already bad storage system even worse.
So I will softly recommend to you to implement a storage backend first.
The file system storage backend should be easy to do, because most code can be copied from the CLI.

If you're not the type to finish what you've started, you can always do something new instead!
Here's the guide on [where to go next](./where_next.md).

There's one more thing to consider when implementing these additional features for the HTTP server.
If you implement them in a way that they are _required_, then that means all you client components will have to implement the feature before being compatible with your HTTP server.
For example, your server might _require_ that a client provide a receiver when sending a paekli.
If you want to ensure that other clients can integrate as quickly as possible, even without the full feature set, make sure to keep these features _optional_.
In the case of the receiver, your HTTP server could default to some shared inbox where everyone can send and receive paekli without identification.

## Expanding our storage space

Requirements:
- Multiple paekli can be sent before they are received.
- Paekli are received in the same order as they were sent.

## Individual recipients

Requirements:
- Paekli can be sent to a specific recipient
- Receivers of a paekli can identify themselves and only receive paekli intended for them.

## Express delivery

Requirements:
- Paekli can optionally be sent with express delivery
- Express paekli are always received before non-express paekli.

```admonish check title="Release"
Don't forget to cut a release once you've implemented these features!
```
