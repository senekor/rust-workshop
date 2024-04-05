# Content and Storage

```admonish info
This guide assumes you have created the [storage backend abstraction](/storage_backend.md).
If you haven't, I recommend you do it now.
```

Accepting input from Python is simple enough, you just declare the parameters in the function that you need.
It's just like writing regular Rust.

To express that the function might fail but there's no output in the success case, we can use `PyResult<()>` as the return type.

```rust
fn send(_content: &str) -> PyResult<()> {
    Ok(())
}
```

Are you wondering about how it's possible that we're simply writing Rust code, including input parameters and return types, and Python can just call those functions?

The magic lies with the `#[pyfunction]` macro, which generates the glue code necessary to convert between Python and Rust values across the FFI boundary.
You can make your own Rust types able to pass the FFI boundary to Python by implementing the traits `FromPyObject` (derivable) and `IntoPy<PyObject>`.
If you're curious, you can read more about these conversions [here](https://pyo3.rs/v0.21.0-beta.0/conversions).

## Storing paekli

Using our storage backend abstraction, we'll be done with this in no time.
Remember that you need to add `paekli-core` as a path dependency to `paekli-py`.

You should have a function to get access to a `DistributionCenter`, e.g. `paekli_core::new_distribution_center()`.
If you have more than one storage backend already (or a configurable one), that function probably takes additional arguments.

Next, you just call `.store()` or `.retrieve()` on the distribution center with the arguments received from the Python code.
Easy-peasy.

```admonish success
That was super fast!
PyO3 and our storage backend did almost all the work.

If you have the CLI already, you should be able to send and receive paekli between it and Python (assuming the same storage backend).
```
