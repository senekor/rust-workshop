```yaml
layout: cover
class: text-center
```

# Practice

Ready your laptops!

I will quickly explain all the setup steps.

You'll receive step-by-step instructions in writing as well.

<Nr />

---

```yaml
layout: center
```

![](/fork.png)

<div
    style="border-color: red"
    class="border-4 absolute top-38.5 left-78 w-58 h-9"
></div>

<div
    style="border-color: red"
    class="border-4 absolute top-71.5 left-194.5 w-35.5 h-10.8"
></div>
<Arrow color="red" x1="780" y1="420" x2="820" y2="350" />

<Nr />

---

```yaml
layout: center
class: text-center
```

![](/install-page.png)

<div
    style="border-color: red"
    class="border-4 absolute top-8.2 left-92 w-21 h-8"
></div>
<Arrow color="red" x1="300" y1="120" x2="360" y2="80" />

<div
    style="border-color: red"
    class="border-4 absolute top-20 left-120 w-18 h-10"
></div>
<Arrow color="red" x1="400" y1="120" x2="460" y2="105" />

<Arrow color="red" x1="100" y1="456" x2="180" y2="456" />

<Nr />

---

```yaml
layout: center
class: text-center
```

# Batteries included

| **component** | **purpose**             | **example**                 |
| ------------: | :---------------------- | :-------------------------- |
|        rustup | toolchain manager       | `rustup update`             |
|         cargo | package manager         | `cargo add my-fav-library`  |
|         rustc | compiler                | `cargo run` , `cargo build` |
|       rustdoc | documentation generator | `cargo doc --open`          |
|       rustfmt | formatter               | `cargo fmt`                 |
|        clippy | linter                  | `cargo clippy`              |
| rust-analyzer | LSP implementation      | N/A                         |

<Nr />

---

```yaml
layout: center
class: text-center
```

# Ensure you have a linker

|   **System** | **install command**      |
| -----------: | :----------------------- |
| Debian-based | `sudo apt install gcc`   |
|        MacOS | `xcode-select --install` |

<Nr />

---

```yaml
layout: center
class: text-center
```

# Initialize, compile and run a Rust project

```
$ cargo new hello
    Creating binary (application) `hello` package

$ cd hello
$ ls --tree
.
├── Cargo.toml
└── src
   └── main.rs

$ cargo run
Hello, world!
```

<Nr />

---

```yaml
layout: center
```

# Visual Studio Code Extensions

recommendations cover:

- syntax-highlighting
- autocomplete
- diagnostics
- debugging
- `toml` syntax-highlighting

<Nr />

---

```yaml
layout: center
class: text-center
```

<img
    src="/vscode-clippy.png"
    class="w-80%"
/>

<div
    style="border-color: red"
    class="border-4 absolute top-18 left-18 w-52 h-9.3"
></div>

<div
    style="border-color: red"
    class="border-4 absolute top-106 left-68 w-72 h-24"
></div>

<Nr />

---

```yaml
layout: cover
class: text-center
```

# Exercise Organization

|                  |                                        |
| ---------------: | :------------------------------------- |
| custom exercises | in the repo you forked                 |
|        rustlings | download a folder, e.g. inside fork    |
|         Exercism | online editor or need to configure CLI |
|   Advent of Code | locally wherever you want              |

<Nr />

---

```yaml
layout: cover
class: text-center
```

# Practice 🧑‍💻

## [github.com/senekor/rust-exercises](https://github.com/senekor/rust-exercises)

<div class="h-8"></div>

## [`rust-exercises/day_1/README.md`](https://github.com/senekor/rust-exercises/blob/main/day_1/README.md#day-1)

<Nr />
