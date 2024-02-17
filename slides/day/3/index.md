```yaml
favicon: rust-logo-64x64-blk.png
transition: slide-left
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
transition: slide-left
```

# What I left out from the book

- Things that are easy to pick up along the way.\
  (counter example: turbo fish `::<>` is impossible to google)
- Topics you're unlikely to run into at the start of your Rust journey.\
  (smart pointers, interior mutability, dynamic dispatch,\
  concurrency (`Send`, `Sync`), `unsafe` Rust, macros...)

You should still read the book at your own pace if you're serious about learning Rust!

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Hint for Practice Session

```rust {all|3|6|all}
// a test that ensures an expected panic occurs
#[test]
#[should_panic]
fn expected_panic_occurs() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}
```

---

```yaml
layout: cover
class: text-center
transition: slide-left
```

# Practice 🧑‍💻

## `rust-workshop/day_3/README.md`

---

```yaml
layout: center
class: text-center
```

# Please suggest improvements
# for next week!
# 🦀

Check the readme of your repository for the form link.
