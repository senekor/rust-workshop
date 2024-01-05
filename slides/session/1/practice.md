```yaml
layout: cover
class: text-center
transition: slide-up
```

# Practice

Ready your laptops!

This first session will start slow, as we make sure everyone has a functioning setup.

The following practice sessions will allow a more individual learning pace.

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

---

```yaml
layout: center
transition: slide-up
```

![](/repo-template.png)

<div
    style="border-color: red"
    class="border-4 absolute top-39 left-70 w-57 h-9"
></div>

<div
    style="border-color: red"
    class="border-4 absolute top-72 left-194 w-34.4 h-10"
></div>
<Arrow color="red" x1="780" y1="420" x2="810" y2="350" />

---

```yaml
layout: center
transition: slide-up
```

# `rust-workshop/session_1/index.md`

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

---

```yaml
layout: center
class: text-center
transition: slide-left
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
