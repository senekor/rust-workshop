```yaml
layout: cover
class: text-center
transition: slide-up
```

# Enums & Pattern Matching

book chapter 6

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
    home.address[8]; // ??
}
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-72.5 left-84 w-16"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-97.5 left-93 w-26.5"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-107.5 left-90 w-6"
></div>

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

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Billion-Dollar Mistake

<div></div>

<img
    src="/option_meme.png"
    style="height: 30vh"
/>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# The `Option` Type

```rust {1-4,6,7,9}
enum Option<T> {
    None,
    Some(T),
}
main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Preventing Bugs

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y; // error
```

compiler says:

> cannot add `Option<i8>` to `i8`

---
src: ./match.md
---
