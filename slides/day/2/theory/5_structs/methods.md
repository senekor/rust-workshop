```yaml
layout: cover
class: text-center
transition: slide-up
```

# Methods

book chapter 5.3

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
    class="h-0.8 rounded absolute top-62 left-70 w-8"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-67 left-96 w-11"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
transition: slide-up
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
    class="h-0.8 rounded absolute top-54 left-93 w-52"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-90.5 left-110 w-3"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
    let sq = Rectangle::square(3);
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-56.5 left-145 w-15"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
transition: slide-left
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
