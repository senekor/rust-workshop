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

# The Rust Ecosystem

<div></div>

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>Libraries & Documentation</li>
    <li>Idiomatic APIs</li>
    <li>Developer Tools</li>
    <li>Testing</li>
    <li>Continuous Integration & Delivery</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />

---
src: 1_libs_docs.md
---

---
src: 2_rusty_apis.md
---

---
src: 4_dev_tools.md
---

---
src: 5_testing.md
---

---
src: 6_ci_cd.md
---

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
$ paekli-cli receive
I don't have any paekli for you! I'm sorry 😢

$ paekli-cli send --express "Ferris Plushie 🦀"

$ paekli-cli receive
{ "content": "Ferris Plushie 🦀", "express": true }
```

Now imagine you send a paekli via CLI and it automatically\
arrives in the web app, pushed via websocket 🤯

<Nr />

---

```yaml
layout: cover
class: text-center
```

# Practice 🧑‍💻

## [`rust-exercises/day_5/README.md`](https://github.com/senekor/rust-exercises/blob/main/day_4/README.md#day-4)

<div style="height: 3em"></div>

# Work on Projects

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <table>
      <thead>
        <tr></tr>
      </thead>
      <tbody>
        <tr>
          <td>
            paekli-rs
          </td>
          <td>
            <a href="https://senekor.github.io/paekli-rs">senekor.github.io/paekli-rs</a>
          </td>
        </tr>
        <tr>
          <td>
            LED-Matrix
          </td>
          <td>
            <a href="https://github.zhaw.ch/senk/led-matrix-rs-template">github.zhaw.ch/senk/led-matrix-rs-template</a>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
  <div style="flex-grow: 1"></div>
</div>


<Nr />
