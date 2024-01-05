```yaml
layout: cover
class: text-center
transition: slide-up
```

# Structs & Methods

book chapter 5

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Declaration

```rust
struct Rectangle {
    width: u32,
    height: u32,
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
let rect = Rectangle {
    width: 200,
    height: 100,
};
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Field Access

```rust
rect.width = 120;
let area = rect.width * rect.height;
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Tuple Structs

```rust
struct Color(u8, u8, u8);

fn main() {
    let color = Color(12, 200, 85);
    let green = color.1;
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Tuple Structs: Encapsulation

```rust
struct NonZeroByte(u8);

fn into_non_zero_byte(raw: u8) -> NonZeroByte {
    if raw == 0 {
        panic!("You had one job!")
    }
    NonZeroByte(raw)
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Unit-Like Structs

```rust
struct OnePossibleValue;

fn main() {
    let seems_useless: OnePossibleValue = OnePossibleValue;
}
```

Extremely useful with traits!

Sit tight for day 3.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Ownership of Struct Data

```rust
struct User {
    name: &str,
}
```

It's possible, but don't try it yet!

Sit tight for day 3.

<div
    style="background-color: red"
    class="h-0.5 absolute top-66 left-104 w-9"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Deriving Traits

```rust {1,2,8-10}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect has the value: {:?}", rect);
    //                            ^^^^
    //     note the debug-print syntax
}
// output:
// rect has the value: Rectangle { width: 30, height: 50 }
```

Sit tight for traits on day 3! 😄

---
src: ./methods.md
---
