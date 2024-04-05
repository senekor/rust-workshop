# Command Line Argument Parsing

The library we'll be using to write our CLI is called `clap`.
You can find its documentation as usual on [docs.rs/clap](https://docs.rs/clap).
There are other libraries to parse CLI-arguments, but `clap` is the most popular and user-friendly one.
The alternatives focus on fast compile times, which isn't a top-priority for `clap`.

## Adding the dependency

A small stumbling block for Rust's dependency management can be libraries with missing feature flags.
Helpfully, the output of `cargo add` shows us a list of included and excluded features.
I recommend to check this list every time you add a dependency and look for missing features you may want to include as well.
The coolest feature of `clap` is also its most compile-time heavy one, so it is gated behind a non-default feature flag called `derive`.

Let's add `clap` to our dependencies:

```sh
# cd paekli-cli
cargo add clap --features derive
```

If we forget about `--features derive`, some of the later code won't compile, and it wouldn't be obvious why.
We can confirm that it worked by checking that `paekli-cli/Cargo.toml` now contains the following:

```toml
[dependencies]
clap = { version = "4.5.1", features = ["derive"] }
```

## A barebones `clap`-app

Here is the absolute minimum code to use clap:

```rust
use clap::Parser;

#[derive(Parser)]
struct Cli;

fn main() {
    let _args = Cli::parse();
    println!("Paekli LLC is currentli closed 😢");
}
```

What we're doing here is defining our CLI as a _data type_.
If you think about it, the structure of a standard CLI can easily be represented as a native Rust type.
For now, our `struct Cli;` is empty and therefore doesn't accept any arguments.

To parse the actual command line arguments into this data structure, we `#[derive(Parser)]` on it, which is a macro we imported with `use clap::Parser;`.
Finally, we simply call `Cli::parse();` in our main function.

If we run this program, we should get the same output as before.
So you'd be forgiven to think that didn't accomplish anything.
However, `clap` generates a help page for you, even if you don't specify a single CLI argument.
You can see it by running:

```sh
cargo run --quiet -- --help
```

Note how `--` is used to distinguish between the arguments passed to `cargo` and the ones passed to your program.
If we had a pre-compiled binary, we could simply run `paekli-cli --help`.

```
> cargo run --quiet -- --help
Usage: paekli-cli

Options:
  -h, --help  Print help
```

It's great to have a standard help page, as is expected from every CLI tool.
It will automatically be kept up-to-date with the structure of our `Cli` data type, courtesy of Rust's powerful macros.

It is also good practice to let your users check which version of your program their running.
You wouldn't want to get bug reports for outdated software!
Change your `Cli` definition like so, to add a version flag:

```rust
#[derive(Parser)]
#[clap(version)]
struct Cli;
```

Now oberseve the output of:

```sh
cargo run --quiet -- --version
cargo run --quiet -- --help
```

We should also provide at least a small description of what paekli-cli does to our users.
`clap` automatically includes your doc-comments in the output to users, so let's add one to our CLI:

```rust
/// send and receive joy with ✨ paekli-cli ✨
#[derive(Parser)]
#[clap(version)]
struct Cli;
```

Note the tripe-slash `///`, which distinguished a doc-comment from a regular comment.
The comment should now show up on the help page.

Lovely!
That's a great foundation to build an enjoyable CLI on top of.

````admonish check title="Release"
Our users can now get information about the purpose of the app, how to use it and the version they're running.
That's definitely worthy of a new release!
Recall the process:

```
update Cargo.toml ; commit ; push ; tag v0.1.X ; push --tags
```
````
