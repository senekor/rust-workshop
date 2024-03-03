```yaml
layout: cover
class: text-center
transition: slide-up
```

# Enums & Pattern Matching

book chapter 6

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Declaration

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Instantiation

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Storing Data ?

```rust {5-8,10-14}
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: [u8; 16], // v4 and v6 both fit in here
}
fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: [127, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    home.address[8]; // ???
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-72 left-77 w-17"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-102 left-87 w-31"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-114 left-83.5 w-6"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Enums are Tagged Unions

```rust {1-4|6|8}
enum IpAddr {
    V4([u8; 4]),
    V6([u8; 16]),
}
fn main() {
    let home = IpAddr::V4([127, 0, 0, 1]);

    home.0[8]; // nice try :-)
}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Flexible Data Modelling

```rust
enum Message {
    Quit,                       // ~= unit-like struct
    Move { x: i32, y: i32 },    // ~= regular struct
    Write(String),              // ~= tuple struct
    ChangeColor(i32, i32, i32),
}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Billion-Dollar Mistake

<div></div>


<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    The C pointer mixes two unrelated concepts:
    <li>Indirection</li>
    <li>Optionality (something or nothing)</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<div class="h-4"></div>

> I call it my billion-dollar mistake.
> It was the invention of the null reference in 1965.
>
> _Tony Hoare_

<div class="h-4"></div>

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: center">
    Rust:
    <table>
      <tr>
        <td>Indirection:</td>
        <td>Reference <code>&</code></td>
      </tr>
      <tr>
        <td>Optionality:</td>
        <td>❓</td>
      </tr>
    </table>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# The `Option` Type

```rust {1-4,6,7,9}
enum Option<T> {
    None,
    Some(T),
}
main() {
    let some_number = Option::Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

<Nr />

---
src: ./match.md
---
