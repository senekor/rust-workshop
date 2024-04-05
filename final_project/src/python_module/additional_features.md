# Additional Features

Now you have the opportunity to bring the Python extension module to feature-parity if you have already implemented the additional features for another component.

As your Python functions accumulate more parameters, you may want to use optional and keyword arguments, which are idiomatic in Python.
I have good news for you, there is little work to be done to achieve that.
From the [PyO3 documentation](https://pyo3.rs/v0.21.0-beta.0/function/signature):

> Like Python, by default PyO3 accepts all arguments as either positional or keyword arguments.
> Most arguments are required by default, except for trailing `Option<_>` arguments, which are implicitly given a default of `None`.

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
