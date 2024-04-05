# Additional Features

As we go along building our application, we will quickly want to add more features.
How to implement them for the CLI will be described in this section.
You will be mostly on your own, but guidance will be given where new concepts / libraries etc. are required.

You can choose to skip this section for now and explore the other commponents and integrations.
Just remember to come back here if some integration requires you to have these features implemented.
Jump to the [next section](where_next.md) to explore other components and integrations or keep reading to implement more CLI features.

Once you're happy with the feature set of the CLI, don't forget to cut a new release!

## Expanding our storage space

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

## Individual recipients

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

## Express delivery

Our paekli are currently always received in FIFO order.
However, what if some paekli was really important?
For example, a paekli containing a programmable ergonomic split mechanical keyboard with no less than eight keys on each thumb cluster?
Surely our users would like to receive such a marvelous paekli before all the other ones.

This feature will nicely demonstrate a boolean flag.
To implement one with `clap`, do the same as with a regular flag, but use a `bool` as its type instead of `String`.
The existence of the flag on the command line represents `true`.

The rest is up to you!

```admonish check title="Release"
Now that our CLI is jam-packed with exciting features, it's time for the next release.

Future releases likely won't add significant new features, but maybe our CLI will grow to interact with other components!
```
