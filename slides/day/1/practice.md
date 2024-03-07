```yaml
layout: cover
class: text-center
transition: slide-up
```

# Practice

Ready your laptops!

I will quickly explain all the setup steps.

You'll receive step-by-step instructions in writing as well.

<Nr />

---

```yaml
layout: center
transition: slide-up
```

![](/fork.png)

<div
    style="border-color: red"
    class="border-4 absolute top-36.5 left-81 w-62 h-9"
></div>

<div
    style="border-color: red"
    class="border-4 absolute top-72.5 left-156 w-35.5 h-10.8"
></div>
<Arrow color="red" x1="780" y1="420" x2="740" y2="350" />

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
transition: slide-up
```

# Choose your `$PATH`

should be handled by the install script

Make sure you you can call `cargo`.\
If you can't, try some of these:

```bash {lineNumbers: false}
# POSIX shell
source ~/.cargo/env
# or
export PATH="$HOME/.cargo/bin:$PATH"
```

```sh {lineNumbers: false}
# fish
fish_add_path ~/.cargo/bin
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Batteries included

|     component | purpose                 | example                     |
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
transition: slide-up
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
transition: slide-up
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
transition: slide-up
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
transition: slide-left
```

# Practice 🧑‍💻

## [github.com/senekor/rust-workshop](https://github.com/senekor/rust-workshop)

<div class="h-8"></div>

## [`rust-workshop/day_1/README.md`](https://github.com/senekor/rust-workshop/blob/main/day_1/README.md#day-1)

<Nr />
