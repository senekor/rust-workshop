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

## Day 1

---

```yaml
layout: center
class: text-center
```

# Why Rust?

<div style="height: 2em"></div>

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left; font-size: 1.3em">
    ✅ performance <br/>
    ✅ safety <br/>
    ✅ correctness <br/>
    ✅ productivity <br/>
    ✅ portability <br/>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />

<!--
Rust has the same performance ceiling as C/C++.
Inline assembly is possible

Rust is memory safe like most garbage collected languages.
Unlike most languages, it even prevents data-races in multi-threaded environments.

Beyond safety aspects, Rust has many features that help you ensure your program does what it's supposed to.
- no null-pointers
- no implicit type conversions
- algebraic data types

Rust has excellent productivity tooling:
- type inference
- automatic formatting
- great IDE support (via LSP)
- package manager

From bare-metal embedded to WebAssembly in the browser - Rust can comfortably run anywhere unlike any other language
-->

---

```yaml
layout: center
class: text-center
```

# Program

<div></div> <!-- prevent subheading -->

|       |                      |                                                          |
| :---- | -------------------: | :------------------------------------------------------- |
| Day 1 |    Language Basics 1 | common programming concepts & ownership                  |
| Day 2 |    Language Basics 2 | structs, enums, modules, collections, error handling     |
| Day 3 |  Advanced Features 1 | generics, traits, lifetimes, closures, iterators         |
| Day 4 |  Advanced Features 2 | smart pointers, dynamic dispatch, async programming      |
| Day 5 |   The Rust Ecosystem | libraries, documentation, patterns, CI/CD, project start |
| Day 6 |          Projects 🚀 | CLI tools, web APIs, python modules, LED matrix          |

<Nr />

---

```yaml
layout: center
class: text-center
```

<a href="https://doc.rust-lang.org/stable/book/"><img src="/book_cover.png" style="height: 70vh"/></a>

<Nr />

---

```yaml
layout: center
class: text-center
```

# About these slides

<br/>

Although they aren't meant as reference material...\
Both the interactive version and PDF exports are available at:

[senekor.github.io/rust-workshop](https://senekor.github.io/rust-workshop)

<br/>

Using the interactive slides:\
Hover over the bottom-left corner for controls.

<Arrow x1="200" y1="450" x2="120" y2="500" color="green" width="3" />

<Nr />

---
src: ./theory/index.md
---

---
src: ./practice.md
---

---

```yaml
layout: center
class: text-center
```

# Please suggest improvements
# for next week!
# 🦀

Check the readme of your repository for the form link.
