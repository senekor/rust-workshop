```yaml
layout: cover
class: text-center
```

# Methods

book chapter 5.3

<Nr />

---

```yaml
layout: center
class: text-center
```

# Syntax

```rust {5-9,12,14}
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 { // area(self: &Rectangle)
        self.width * self.height
    }
}
fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let a: u32 = rect.area();
    // this works:
    let a: u32 = Rectangle::area(&rect);
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-62 w-9.5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-66 left-91 w-14"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Automatic (De-)Referencing

```rust
fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    
    println!("{}", rect.area()); // adds 1 reference

    let rect: &&Rectangle = &&rect;
    println!("{}", rect.area()); // removes 1 reference

    let rect = &&&&&&&&rect;
    println!("{}", rect.area()); // ok we get it
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Additional Parameters

```rust {2-4,7-9}
impl Rectangle {
   fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect_1: Rectangle;
    let rect_2: Rectangle;
    if rect_1.can_hold(&rect_2) {
        // ...
    }
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-51 left-88 w-59.5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-93 left-107 w-3.5"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Associated Functions

```rust {2-7,10}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let square = Rectangle::square(3);
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-54 left-143 w-17"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Multiple `impl` Blocks

```rust {1-2,4-7,12-13}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Summary

structs and methods

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>group data meaningfully</li>
    <li>combine data with related behavior</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />
