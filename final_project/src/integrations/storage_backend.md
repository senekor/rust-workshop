# Storage Backend

Most of the components use identical methods to store paekli, but the method itself may be configurable.
For example, both the CLI and the python module might store paekli in the file system, an SQL database or delegate storage to the HTTP server.
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

The fundamental operations `store` and `retrieve` are obvious, but I'll let you decide what the parameters and return types should be.
Don't worry about getting it right immediately, you can always refactor the interface as the need arises.
Rust's strong type system will help you correctly change all the places where the interface is already used.

Since this `DistributionCenter` will be used by most of the components, it needs to go into our `paekli-core`.
Read the instructions for [creating a shared library](shared_lib.md) if you haven't already.

```admonish title="The first implementation"
Since you probably already implemented file system storage for the CLI component, it makes sense to turn that into your [first implementation of the trait](/components/file_system.md).

For the next section to make sense, make sure you have at least one implementation.
```

## Using the trait

The first time you're reading this, you will only have one storage backend.
It would be simple to just use that directly.
To future proof for additional storage backends, let's assume we already have two:

```rust
// paekli-core/src/storage.rs
trait DistributionCenter {}

struct FileSystemStorage;
impl DistributionCenter for FileSystemStorage {}

struct HttpClient;
impl DistributionCenter for HttpClient {}
```

Let's try to write a simple funtion for creating a `DistributionCenter`:

```rust
fn new_distribution_center(local: bool) -> ? {
    if local {
        FileSystemStorage
    } else {
        HttpClient
    }
}
```

Hm, this doesn't quite work.
There is no way for us to write a correct return type for this function, because the two possible values have different types.

What we need is _dynamic dispatch_:

```rust
fn new_distribution_center(local: bool) -> Box<dyn DistributionCenter> {
    if local {
        Box::new(FileSystemStorage)
    } else {
        Box::new(HttpClient)
    }
}
```

```admonish info title="How dynamic dispatch works" collapsible=true
The full explanation of dynamic dispatch can be found in [chapter 17 of the Rust book](https://doc.rust-lang.org/book/ch17-02-trait-objects.html).
Here's a condensed version:

The `dyn` keyword generates a table of function pointers (**vtable**) for each type that implements a certain trait.
The vtable contains pointers to all methods of the trait and the `drop` method (destructor).

A pointer to this vtable is then stored _alongside_ a pointer to the actual value.
Such a value is often referred to as a _trait object_.
It uses the same mechanism of storing metadata with a pointer (fat pointer) as the slice type.

This means that dynamic dispatch only works with some kind of pointer type, like `Box<dyn Foo>` or `&dyn Foo`.
The _memory size_ of the value behind the poiner is unknown at compile time.
That's OK though, because the only thing you can do with such a value is call methods on it that actually do know its size.
```

Finally, in your clients that need to access a distribution center, you can write:

```rust
let center = new_distribution_center();
center.store();
center.retrieve();
```
