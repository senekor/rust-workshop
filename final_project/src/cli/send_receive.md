# Sending and Receiving Paekli

It's time to build our first feature.
The most basic service we offer to our customers is _sending_ and _receiving_ paekli.
How should we model this in the CLI?
The most natural choice is the **subcommand**, which is often used by programs that offer many different functionalities.
For example: `add`, `commit` and `push` are subcommands of `git`.

```admonish info
The code snippets in this guide will become less complete as we go along.
It is your responsibility to make sure the things you copy-paste integrate correctly with the rest of your code.
This is also because you are encouraged to add, modify and experiment with things according to your whim and curiosity.
It is **your** final project after all! 😃
```

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

```admonish check title="Release"
Phew!
We've made some great progress.
Let's cut a new release to let our users enjoy this new feature.
```
