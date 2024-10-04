```yaml
favicon: rust-logo-64x64-blk.png
transition: slide-up
titleTemplate: '%s'
lineNumbers: true
fonts:
  mono: 'Fira Mono'
class: text-center
```

# Rust Workshop

## Day 3

---
src: ./theory/index.md
---

---

```yaml
layout: cover
```

# What I left out from the book

- Things that are easy to pick up along the way.\
  (counter example: turbo fish `::<>` is impossible to google)
- Topics you're unlikely to run into at the start of your Rust journey.\
  (smart pointers, interior mutability, dynamic dispatch,\
  concurrency (`Send`, `Sync`), `unsafe` Rust, macros...)

You should still read the book at your own pace if you're serious about learning Rust!

<Nr />

---

```yaml
layout: center
class: text-center
```

# Hint for Practice Session

```rust {all|3|6|all}
// a test that ensures an expected panic occurs
#[test]
#[should_panic]
fn expected_panic_occurs() {
    let v = vec![1, 2, 3];
    v[10];
}
```

<Nr />

---

```yaml
layout: cover
class: text-center
```

# Practice 🧑‍💻

## [`rust-exercises/day_3/README.md`](https://github.com/senekor/rust-exercises/blob/main/day_3/README.md#day-3)

<Nr />
