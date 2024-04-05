# Sending and Receiving Paekli

Let' start implementing the two basic interactions with the system.
They're just going to be two stubbed-out functions for now.
Remember to add them explicitly to your module!

```rust
/// Send a paekli
#[pyfunction]
fn send() -> &'static str {
    SEND_MESSAGE
}
#[pymodule]
fn paekli_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send, m)?)?;
}
```

That was easy!
You should be able to confirm your changes with something like:

```python
import paekli_py

print(paekli_py.send())
print(paekli_py.receive())
```
