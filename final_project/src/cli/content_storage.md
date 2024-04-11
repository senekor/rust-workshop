# Content and Storage

At this point, our users can send and receive paekli.
However, they probably want to send paekli with some content and they want _that content to be received_.
Silly users with their unrealistic feature requests!

But let's try to make them happy.

## Sending paekli with content

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

## Storing paekli for delivery

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

## Less than terrible error handling

We now have a few calls to `.expect()` in our code.
This is great for whipping up a quick program that works, but it immediately crashes our program in case of an error.
There are libraries for more scaleable error handling with great usability.
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
Lastly, we append the question mark operator `?` to the call of `.context()` in order to _return early_ in case of an error.

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
Remember to remove the file, otherwise a paekli could be delivered twice!

```admonish check title="Release"
Contratulations!
We now have a fully-functioning minimum viable product (MVP).
The basic functions of sending and receiving work as expected.

Pat yourself on the back and cut a new release! 🥳
```
