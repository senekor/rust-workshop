# Creating a Shared Library

For multiple reasons, our different components may need to share some code.
Luckily this is very simple.
Let's start by initializing a new _library_ package:

```sh
cd rust-workshop/final_project
cargo new --lib paekli-core
```

This new library can be used in any of our other packages by adding it as a dependency first.
Note that we need the `--path` flag for a local library, as opposed to ones from crates.io.

```sh
cd final_project/paekli-cli
cargo add --path ../paekli-core
```

Lastly, you can confirm it works by trying to import the generated `add` function.
Note how the dash in `paekli-core` changes to an underscore in Rust code.
This is automatic, because Rust identifiers cannot contain dashes.

```rust
// paekli-cli/src/main.rs
use paekli_core::add;
```

Now you can add some functionality to `paekli-core` and use it in your other packages.
