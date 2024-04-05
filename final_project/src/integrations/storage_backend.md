# Storage Backend

Most of the components use identical methods to store paekli, but the method itself may be configurable.
For example, both the CLI and the python module might store paekli in the file system, a sqlite database or delegate storage to the HTTP server.
The server itself may use the file system, a database _or_ just keep it in-memory... you get the point.

This is the perfect place to introduce an abstraction.
We can program a component to work independently of how paekli are stored.
Different storage methods can then be swapped out easily.

If we think about a postal service, we might make the analogy of a distribution center.
The postal office doesn't need to know how the distribution center works, it only cares about the functionality it provides.

This is a perfect use case for a `trait`:

```rust
trait DistributionCenter {
    fn store();
    fn retrieve();
}
```

The fundamental operations `store` and `receive` are obvious, but I'll let you decide what the parameters and return types should be.
Don't worry about getting it right immediately, you can always refactor the interface as the need arises.
Rust's strong type system will help you correctly change all the places where the interface is already used.

Since this `DistributionCenter` will be used by most of the components, it needs to go into our `paekli-core`.
Read the instructions for [creating a shared library](shared_lib.md) if you haven't already.

```admonish title="The first implementation"
Since you probably already implemented file system storage for the CLI component, it makes sense to turn that into your [first implementation of the trait](/components/file_system.md).

For the next section to make sense, make sure you have at least one implementation.
```

## Using the trait

TODO
