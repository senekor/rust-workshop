# CLI

Oh, hi there!
I'm happy you came by ❤️ I guess that means you want to write a `paekli-cli` with me?
Awesome, let's get started!

## Initializing a new package

We're going to keep our CLI in the directory `rust-workshop/final_project/`.
Here are the commands to let cargo bootstrap it:

```sh
cd rust-workshop/final_project
cargo new paekli-cli
```

You should now have two new files:
- `paekli-cli/Cargo.toml`
- `paekli-cli/src/main.rs`

...as is the case with any new executable Rust package.

Also note that the `members` key of the top-level `Cargo.toml` should have been modified automatically to include your new package:

```toml
# rust-workshop/Cargo.toml
members = ["day_?/*", "final_project/paekli-cli"]
```

That's perfectly fine.
We didn't worry much about _cargo workspaces_ in this workshop, which is what that top-level `Cargo.toml` defines.
They simply give you some quality-of-life improvements for managing multiple packages in a single project / repository.

## Shipping the first version

Before we add any features, we need to make sure we can ship our software efficiently.
Let's just change the print statement in the main function for now:

```rust
fn main() {
    println!("Paekli LLC is currentli closed 😢");
}
```

Remembering the release-worklfow we've already seen during the workshop, all we have to do is commit our changes and push a new version tag.
You have probably already "used" some version tags in your repository, so just pick the next higher one to release the first version of paekli-cli, for example `v0.1.2`.

```toml
# paekli-cli/Cargo.toml
version = "0.1.2"
```

```sh
git add --all
git commit
git push
git tag v0.1.2
git push --tags
```

That should be it!
It's still a good idea to keep an eye on the release job on GitHub and try out the finished executable manually.

## `clap` - the Command Line Argument Parser

The library we'll be using to write our CLI is called `clap`.
You can find its documentation as usual on [docs.rs/clap](https://docs.rs/clap).
There are other libraries to parse CLI-arguments, but `clap` is the most popular and user-friendly one.
The alternatives focus on fast compile times, which isn't a top-priority for `clap`.

### Adding the dependency

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

### A barebones `clap`-app

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

```admonish info
The code snippets in this guide will become less complete as we go along.
It is your responsibility to make sure the things you copy-paste integrate correctly with the rest of your code.
This is also because you are encouraged to add, modify and experiment with things according to your whims and curiosity.
It is **your** final project after all! 😃
```

## Sending and receiving

It's time to build our first feature.
The most basic service we offer to our customers is _sending_ and _receiving_ paekli.
How should we model this in the CLI?
The most natural choice is the **subcommand**, which is often used by programs that offer many different functionalities.
For example: `add`, `commit` and `push` are subcommands of `git`.

Subcommands represent a choice out of a finite set of alternatives, that's a perfect fit for an `enum`.
Let's try it:

```rust
#[derive(Subcommand)]
enum Command {
    Send,
    Receive,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
```

The enum definition should be intuitive, but you'll notice the `#[derive(Subcommand)]` annotation.
In the struct `Cli`, we have to add the command as a field, again with a new annotation `#[command(subcommand)]`.

The compiler error messages for libraries that provide such annotation-based functionality are usually not very helpful, because the compiler cannot know what annotations you _should have added_.
For such libraries, it's best to refer to the documentation.
Because the libraries are aware of the bad error messages, they usually have great documentation and `clap` is no exception.

Now we can use the parsed subcommand in our main function:

```rust
const SEND_MESSAGE: &str = "\
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.
* throws your paekli directly in the trash *";

const RECEIVE_MESSAGE: &str = "\
There aren't any paekli for you at the moment.
* tries to hide paekli in the trash can *";

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Send => println!("{SEND_MESSAGE}"),
        Command::Receive => println!("{RECEIVE_MESSAGE}"),
    }
}
```

````admonish check title="Release"
Phew!
We've made some great progress.
Let's cut a new release to let our users enjoy this new feature.
Recall the process:

```
update Cargo.toml ; commit ; push ; tag v0.1.X ; push --tags
```
````

## Content and storage

At this point, our users can send and receive paekli.
However, they probably want to send paekli with some content and they want _that content to be received_.
Silly users with their unrealistic feature requests!

But let's try to make them happy.

### Sending paekli with content

The content of a paekli will be another CLI argument.
Since we only expect content when _sending_ a paekli, we will add the argument to that subcommand.
Specifying the content of a paekli you're receiving doesn't make sense.

```rust
enum Command {
    Send { content: String },
    Receive,
}
```

We will also need to adjust the match-expression to ignore the content in our main function:

```rust
Command::Send { content: _ } => println!("{SEND_MESSAGE}"),
```

Now, try to send a paekli without content and see how the error message helps the user figure out how to use the CLI correctly.

### Storing paekli for delivery

Applications are expected to store their data in different locations depending on the operating system.
We might be tempted to tell our users to just install Linux when they're bugging us about supporting their platform.
Instead, let's use the [directories](https://docs.rs/directories) crate to not have to worry about it at all.

Here's the code we'll need to add:

```rust
let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
    .expect("the user's home directory seems to be corrupt");
let storage_dir = project_dir.data_dir();
std::fs::create_dir_all(storage_dir).expect("failed to create storage directory");

Command::Send { content } => {
    std::fs::write(storage_dir.join("content"), content)
        .expect("failed to store paekli");
}
```

On Linux, you can confirm that a paekli was sent correctly with:

```sh
cat ~/.local/share/paekli/content
```

### Less than terrible error handling

We now have a few calls to `.expect()` in our code.
This is great for whipping up a quick program that works, but it immediately crashes our program in case of an error.
There are libraries for more scalable error handling with great usability.
The most popular one for applications (as opposed to libraries) is [anyhow](https://docs.rs/anyhow), so let's use that.

```rust
fn main() -> anyhow::Result<()> {
    // --snip --

    Ok(())
}
```

`anyhow::Result` is a different type than `Result` from the standard library.
It only takes one type parameter for the `Ok` case.
The `Err` case always holds a value of type `anyhow::Error`.
This makes it easy to bubble up errors when they're all the same type.
Here we are returning `Result<()>`, because we don't return any value in the success case.
That means we now need to return `Ok(())` at the end of main.

```admonish note title="returning Result from main" collapsible=true
You may not have known that the main function can actually return regular values.
This is mostly useful for returning `Result`s, so you can do normal Rust-style error handling in the main function.
However, `main` can technically return any type that implements the [Termination](https://doc.rust-lang.org/stable/std/process/trait.Termination.html) trait.
```

So, how do we refactor our `.expect()` calls to return `anyhow::Result` instead?
It's simple, first we import the trait `anyhow::Context`.
This attaches a new method `.context()` to any `Result` or `Option` to convert them into an `anyhow::Result`.
Recall that this pattern is sometimes called an "extension trait".
Lastly, we append the questionmark operator `?` to the call of `.context()` in order to _return early_ in case of an error.

visually:

```
                               use anyhow::Context;
value.expect("error msg")  ->  value.context("error msg")?
```

The practical difference is not that big yet, but future-you will probably thank us for starting early with good error handling.

```admonish question title="Preventing data loss"
Currently, our app overwrites existing paekli with new ones.
Here's a task you can do on your own:
Check if the file already exists, and if it does, do not overwrite it and notify the user that our storage is full.
To create an ad-hoc error using anyhow, you can use the macro `anyhow::anyhow!`.
```

### Delivering paekli

I'll leave it up to you to deliver the paekli.
Simply read from the file system and print the content to stdout.
Remeber to remove the file, otherwise a paekli could be delivered twice!

```admonish check title="Release"
Contratulations!
We now have a fully-functioning minimum viable product (MVP).
The basic functions of sending and receiving work as expected.

Pat yourself on the back and cut a new release! 🙂
```

## Additional features

As we go along building our application, we will quickly want to add more features.
How to implement them for the CLI will be described in this section.
You will be mostly on your own, but guidance will be given where new concepts / libraries etc. are required.

You can choose to skip this section for now and explore the other commponents and integrations.
Just remember to come back here if some integration requires you to have these features implemented.
Jump to the [next section](#where-to-go-next) to explore other components and integrations or keep reading to implement more CLI features.

Once you're happy with the feature set of the CLI, don't forget to cut a new release!

### Expanding our storage space

Currently we can only store one paekli at a time.
Additional paekli are rejected until the existing one is received.
Instead of storing the paekli in a single file with a hardcoded name, let's store them in a directory instead.
The most obvious way to store multiple paekli is to use the time they were sent as their file name.
For that, you're gonna need a crate for time handling, like `time` or `chrono`.
`time` is very minimal, but sufficient for our use case.
I would've let you figure out how to use it yourself, but its documentation is hard to navigate in my opinion.
Just call `time::OffsetDateTime::now_utc().to_string()` to get the current time as a string.

We _could_ just pick a random paekli out of the ones in storage whenever a paekli is received.
However, let's challenge ourselves by making sure the paekli are received in FIFO order.
The standard library function [read_dir](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html#platform-specific-behavior) does not guarantee to yield directory entries in a platform-independent order.
The crate `walkdir` has a function [sort_by](https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html#method.sort_by), which could come in handy.
However, it should also be simple enough to implement this yourself.

### Individual recipients

When people send paekli, they usually have a specific recipient in mind.
In order to assign each paekli to a specific recipient, we need additional CLI arguments.
The sender of a paekli needs to say who should receive it and the receiver must identify themself.

For the sender, we could just extend the `Send` subcommand to also accept a receiver, like so:

```rust
Send {
    content: String,
    receiver: String,
}
```

This works, and there's nothing terribly wrong with it.
However, CLI arguments defined this way are expected in a specific order.
(Namely the order in which they were defined in the struct).
As the number of arguments grows, it can become hard for users to get the order right.
To alleviate this, we can introduce _flags_, which are basically named arguments.
Because they are named, their order doesn't matter and it's always clear what's going to happen when typing in the commmand.
Using `clap` we can turn an argument into a flag by giving it a _short_ and a _long_ name.
(Or only one of the two, if we prefer.)

```rust
Send {
    content: String,
    #[arg(short, long)]
    receiver: String,
}
```

The receiver can now be specified with `-r NAME`, `--receiver NAME` or `--receiver=NAME`.
It seems reasonable to keep the `content` as a positional argument, as it is the most important part of a paekli.
However, you can turn that into a flag as well if you like.

You could also name the recipient flag `to`, which would enable a usage very close to natural english:

```sh
paekli-cli send "cheddar cheese" --to Elizabeth
```

Renaming can be accomplished in the macro annotation as well:

```rust
Send {
    content: String,
    #[arg(short('t'), long("to"))]
    receiver: String,
}
```

To complete the feature, you will need to add a receiver argument or flag to the `Receive` subcommand as well.
Lastly, you'll need to change how you store and retrieve paekli so you can determine the intended recipient.

I will leave that up to you!

### Express delivery

Our paekli are currently always received in FIFO order.
However, what if some paekli was really important?
For example, a paekli containing a programmable ergonomic split mechanical keyboard with no less than eight keys on each thumb cluster?
Surely our users would like to receive such a marvelous paekli before all the other ones.

This feature will nicely demonstrate a boolean flag.
To implement one with `clap`, do the same as with a regular flag, but use a `bool` as its type instead of `String`.
The existence of the flag on the command line represents `true`.

The rest is up to you!

## Where to go next

This was probably your first component, in which case there aren't any integrations available yet.
So you probably only need to looks at the components section.
The guide for whatever you pick can be found in the sidebar.

### Components

- If the CLI was your first component, the recommended next one is the [HTTP-server](components/http.md).
  It is the simplest server component and unlocks (indirect) integration with any other client component.
- The other server components (WebSocket, gRPC) are more advanced / niche technologies, so you should only choose them if you have a particular interest in them.
- The other client components cannot integrate directly with the CLI.
  Clients need a server component as a middleman.
  If you're fine with that, feel free to implement another client next.
  The author's personal favorite is the [web app](components/web_app.md):
  It's portable, easy to deploy and doesn't require a single line of JavaScript 😎
- Another goal that might be worth working towards:
  Once you have a WebSocket-server and a GUI-client, you can make the GUI live-updating! 🤯

### Integrations

- If you have a server-style component (HTTP, WebSocket, gRPC), you can integrate with them directly.
- The CLI cannot integrate with other native clients directly, but they can be made to play nice together with shared _storage backends_.
  These are separate components that you need to implement first, before integrating the CLI with them.
- The web app cannot be integrated with the CLI at all.
  You need a server-style component (HTTP, WebSocket) between them.


