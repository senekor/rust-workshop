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

```admonish warning title="Target-incompatible dependencies"
As you implement more components, your shared library will accumulate its own dependencies.
Some of these dependencies are not compatible with the web app.
This can lead to **difficult to understand error messages**.
Essentially, the web app will fail to compile because a bunch of stuff was "not found".
This is because it doesn't exist in the browser, where our app will run.
The next section will explain how to deal with this.
If you have no intention of implementing the web app, you can safely ignore it.
```

## Target-specific code and dependencies

Some functionality in `paekli-core` will only be available for components that run in a normal operating system.
This is notably not the case for the web app.

So, we need to be able to include or exclude some functionality of `paekli-core` based on which platform it's being compiled for.

### Compiling code only for specific targets

Let's assume your library is organized in two modules, `wasm_compatible` and `wasm_incompatible`.
In that case, you can skip compiling the incompatible module for wasm-builds like so:

```rust
pub mod wasm_compatible;

#[cfg(not(target_family = "wasm"))]
pub mod wasm_incompatible;
```

Depending on how you organize your code, you may have to add multiple of these annotations.

See also the [`cfg` page of Rust By Example](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) and the [Rust Reference](https://doc.rust-lang.org/reference/conditional-compilation.html).

### Declaring target-specific dependencies

Even if you already excluded your own target-incompatible code as explained above, depencencies declared in `Cargo.toml` will still (fail to) compile.
We need to tell cargo to completely exclude these libraries from the build, based on the target platform.

Take a look at the following example.
The dependency `serde` is compiled for every target, while `tokio` is not compiled for the web app.

```toml
# paekli-core/Cargo.toml
[dependencies]
serde = { version = "1.0.197", features = ["derive"] }

[target.'cfg(not(target_family = "wasm"))'.dependencies]
tokio = { version = "1.36.0", features = ["full"] }
```

Note how the `cfg` syntax is identical to the one used in the source code attribute.
The documentation for this is in [the cargo book](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies).
