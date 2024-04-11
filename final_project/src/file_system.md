# File System Storage

Many of the available components can use the file system to store paekli.
In order to avoid code duplication, we'll implement the `DistributionCenter` trait.
If you haven't created this trait yet, do the [Storage Backend](storage_backend.md) guide first and then come back here.

## Shortcut if you have the CLI

You have likely already implemented storage of paekli in the file system for the CLI.
In that case you can mostly copy-paste the functionality from there.

If you didn't implement the CLI, jump ahead to the [next section](#storing-paekli-for-delivery).
The file system related instructions from the CLI guide are duplicated here.

```rust
struct FileSystemStorage;

impl DistributionCenter for FileSystemStorage {
    // ... copy-paste code from CLI ...
}
```

When copy-pasting from the CLI, remember to add any libraries the code uses to `paekli-core` as well, including the required feature flags.
(check `paekli-cli/Cargo.toml`)

Already done?
Read up on [how to use the storage backend](storage_backend.md#using-the-trait) next.

## Storing paekli

Applications are expected to store their data in different locations depending on the operating system.
We might be tempted to tell our users to just install Linux when they're bugging us about supporting their platform.
Instead, let's use the [directories](https://docs.rs/directories) crate to not have to worry about it at all.

Here's a couple lines of code you'll probably need:

```rust
let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
    .expect("the user's home directory seems to be corrupt");

let storage_dir = project_dir.data_dir();

std::fs::create_dir_all(storage_dir).expect("failed to create storage directory");

std::fs::write(storage_dir.join("content"), content)
    .expect("failed to store paekli");
```

On Linux, a paekli will be stored at the following location:

```sh
~/.local/share/paekli/content
```

## Retrieving paekli

Retrieving paekli should be easy:
- Read the same file where the paekli is stored.
- Handle the case when there's no paekli.
- Delete any retrieved paekli so they don't get delivered twice.

## Error handling

I will leave this up to you.
The CLI guide includes some additional instructions about using the `anyhow` crate for error handling.
Whatever you choose to do, it may impact the function signatures in your `DistributionCenter` trait.
That's totally fine, the compiler will help you with any necessary refactoring.

```admonish check title="Done"
Jump over [here](storage_backend.md#using-the-trait) to find out how to use the `DistributionCenter`.
```
