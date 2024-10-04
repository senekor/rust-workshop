```yaml
favicon: rust-logo-64x64-blk.png
transition: slide-up
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

|        client         |                server                | storage |
| :-------------------: | :----------------------------------: | :---: |
| CLI<br/>Python module<br/>web app | HTTP/REST-API<br/>WebSocket | file system<br/>SQL |

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
$ paeckli-cli receive
I don't have any paeckli for you! I'm sorry 😢

$ paeckli-cli send --express "Ferris Plushie 🦀"

$ paeckli-cli receive
{ "content": "Ferris Plushie 🦀", "express": true }
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
