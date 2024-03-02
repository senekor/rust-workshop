```yaml
favicon: rust-logo-64x64-blk.png
transition: slide-left
titleTemplate: "%s"
lineNumbers: true
fonts:
  mono: "Fira Mono"
class: text-center
```

# Rust Workshop

## Day 5

---

```yaml
layout: cover
class: text-center
```

# Final Project 🚀

<div></div>

You learned how to write Rust code during the first three days\
and you had a healthy dose of practice.

Now, the final project is less about how to write Rust code\
and more about getting your Rust-based software into production.

<Nr />

---

```yaml
layout: center
class: text-center
```

# What does production look like?

<div style="height: 16px"></div>

|        client         |           GUI-client           |                server                |
| :-------------------: | :----------------------------: | :----------------------------------: |
| CLI<br/>Python module | web app<br/>TUI<br/>native GUI | HTTP/REST-API<br/>websocket<br/>gRPC |

<!-- future idea: add tauri -->

<div style="height: 16px"></div>

Our final project is to create and ship one or a combination of these.

<Nr />

---

```yaml
layout: center
class: text-center
```

# Simulating a Postal Service

This topic works well for many possible combinations of clients and servers.

basic, CLI-only example:

```txt {lines: false}
$ my-paeckli-cli receive
I don't have any paeckli for you! I'm sorry 😢

$ my-paeckli-cli send --express --title "Ferris Plushie 🦀"

$ my-paeckli-cli receive
{ "title": "Ferris Plushie 🦀", "express": true }
```

Now imagine you send a paeckli via CLI and it automatically\
arrives in the web app, pushed via websocket 🤯

<Nr />

---

```yaml
layout: cover
class: text-center
```

# Work on Project 🧑‍💻

## [rw-fp.buenzli.dev](https://rw-fp.buenzli.dev)

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
