```yaml
layout: cover
class: text-center
```

# Structs & Methods

book chapter 5

<Nr />

---

```yaml
layout: center
class: text-center
```

# Declaration

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Instantiation

```rust
let rect = Rectangle {
    width: 200,
    height: 100,
};
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Struct Update Syntax

```rust {5-8}
let rect = Rectangle {
    width: 200,
    height: 100,
};
let wide_rect = Rectangle {
    width: 400,
// [!code word:..:1]
    ..rect // useful with many fields
};
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Field Access

```rust
rect.width = 120;
let area = rect.width * rect.height;
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Tuple Structs

```rust
struct Color(u8, u8, u8);

fn main() {
    let color = Color(12, 200, 85);
    let green = color.1;
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Tuple Structs: Encapsulation

a.k.a. the newtype pattern

```rust
struct NonZeroByte(u8);

fn into_non_zero_byte(raw: u8) -> NonZeroByte {
    if raw == 0 {
        panic!("You had one job!")
    }
    NonZeroByte(raw)
}
```

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-66 left-108 w-10"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---
src: ./methods.md
---
