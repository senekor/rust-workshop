# Additional Features

Now you have the opportunity to bring the web app to feature-parity if you have already implemented the additional features for another component.
You should be mostly fine on your own with everything we've seen so far.
You'll need more (complicated) signals to store additional data and more user input.

However, half the fun of the web app is integrating it with the HTTP server.
So feel free to implement these additional features as you go along with that integration.

## Expanding our storage space

Requirements:
- Multiple paekli can be sent before they are received.
- Paekli are received in the same order as they were sent.

## Individual recipients

Requirements:
- Paekli can be sent to a specific recipient
- Recipients of a paekli can identify themselves and only receive paekli intended for them.

## Express delivery

Requirements:
- Paekli can optionally be sent with express delivery
- Express paekli are always received before non-express paekli.

```admonish check title="Release"
Don't forget to cut a release once you've implemented these features!
```
