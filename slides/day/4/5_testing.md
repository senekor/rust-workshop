```yaml
layout: cover
class: text-center
transition: slide-up
```

# Testing

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

A function like this can be anywhere.

```rust
#[test]
fn program_is_correct() {
    assert_eq!(2 + 2, 4, "math has stopped working");
}
```

It will be executed by `cargo test`.

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

Do your tests have some shared util code?

```rust
#[cfg(test)]
mod tests {
    fn setup() {}
    fn teardown() {}

    #[test]
    fn program_is_correct() {
        setup();
        // ...
        teardown();
    }
}
```

It is good practice to have a `tests` module.

The module is only compiled during testing,\
due to the `#[cfg(test)]` attribute.

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

You can return `Result`s from your tests.

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

As expected, `Ok` means the test passed, `Err` means it failed.

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Test Organization

|                   |                                                                    |
| ----------------: | :----------------------------------------------------------------- |
|        unit tests | have access to your internals according to normal visibility rules |
| integration tests | only have access to the public API of your library                 |

```txt {lines: false}
├─ src
│  └─ *.rs      <-- unit tests
├─ tests
│  └─ *.rs      <-- integration tests
└─ Cargo.toml
```

Integration tests only work for libraries ( `lib.rs` ) !

➔ It is common even for binaries to be split into `main.rs` and `lib.rs`,\
with `main.rs` being small and simple.

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Documentation Tests


```rust
/// Increments a number by one.
///
/// # Examples
///
/// ```
/// assert_eq!(inc(42), 43);
/// ```
pub fn inc(x: i32) -> i32 {
    x + 1
}
```

<img
    src="/doc_example.png"
    style="position: absolute; top: 170px; left: 630px; height: 150px"
/>

`cargo test` will run this example as a test!

<Nr />
