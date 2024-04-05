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

<Nr />

---

```yaml
layout: cover
class: text-center
transition: slide-left
```

# Outlook

|       |                       |                                                           |
| :---- | --------------------: | :-------------------------------------------------------- |
| Day 4 | The Rust Ecosystem 📦 | libraries, documentation, tools, news, CI/CD              |
| Day 5 | Shippable Projects 🚀 | CLI tools, web APIs, python modules, WASM apps            |
| Day 6 |            Wrap-Up ✅ | finish projects, questions, feedback, self-congratulation |

<Nr />

---

```yaml
layout: cover
class: text-center
transition: slide-left
```

# Performance Challenge

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>3 exercises to maximize performance, a new one unlocks every Friday.</li>
    <li>A benchmark determines the 3 winners who will receive a prize each.</li>
    <li>The deadline for all 3 submissions is the <b>3rd of April</b>.</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<div class="h-8"></div>

<div class="flex justify-center">
<img
  style="height: 160px"
  src="/clark.jpg"
/>
<div class="w-8"></div>
<div class="flex flex-col">
  <div class="flex-1"></div>
  <a class="text-8" href="https://challenge.buenzli.dev">challenge.buenzli.dev</a>
  <div class="flex-1"></div>
</div>
</div>

<Nr />

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
    v[10];
}
```

<Nr />

---

```yaml
layout: cover
class: text-center
transition: slide-left
```

# Practice 🧑‍💻

## [`rust-workshop/day_3/README.md`](https://github.com/senekor/rust-workshop/blob/main/day_3/README.md#day-3)

<Nr />

---

```yaml
layout: center
class: text-center
```

# Please suggest improvements
# for next week!
# 🦀

Check the readme of your repository for the form link.
