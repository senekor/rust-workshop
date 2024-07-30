```yaml
layout: cover
class: text-center
```

# Recap of Day 2

<Nr />

---

```yaml
layout: center
class: text-center
```

# Structs

```rust {1-4|6-9|10|12|13}
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let person = Person {
        name: String::from("John"),
        age: 35,
    };
    println!("Person's name: {}", person.name);
}
struct NonZeroByte(u8);
struct OnePossibleValue;
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Methods

```rust {1-4,11,13|1,5-11,14}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let area = rect.area();
    let square = Rectangle::new_square(size);
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Enums

```rust {1-6|7-10}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
enum Option<T> {
    None,
    Some(T),
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Pattern Matching

```rust {1,2,6|1,3,6|1,4,6|1,5,6|7-9|10-12}
match msg {
    Message::Quit => println!("bye bye!"),
    Message::Write(text) => println!("{}", text),
    Message::Move { x, y } => set_position(x, y),
    _ => {},
}
if let Some(num) = option_of_num {
    println!("number detected: {}", num);
}
while let Some(num) = vec_of_nums.pop() {
    println!("removed from vec: {}", num);
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Project Organization

<div></div>

The **_crate_** is the unit of compilation, `main.rs` or `lib.rs` the _root_ of the crate.

Above the crate: `Cargo.toml` defines a _package_ for the build system (`cargo`).

Inside the crate: Code is structured in a _tree_ of _modules_.

<Nr />

---

```yaml
layout: center
class: text-center
```

# Modules & Visibility

```rust {1|2-4|5-6|7}
pub fn carrot() {}
fruits::orange()                // relative
crate::garden::fruits::orange() // absolute
super::garden::fruits::orange() // backwards
use std::collections::HashMap;
let m: HashMap; // type is now in scope
use std::collections::*;
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Error Handling

```rust {1-4|5-9,11|5,10-11|13-17}
enum AreaError {
    BadSeparator,
    BadInteger(String),
}
fn calculate_area(input: &str) -> Result<usize, AreaError> {
    let (left, right) = match input.split_once('x') {
        Some(t) => t,
        None => return Err(AreaError::BadSeparator),
    };
    Ok(parse_int(left)? * parse_int(right)?)
}
fn main() {
    match calculate_area(input) {
        Ok(area) => println!("the area is: {}", area),
        Err(AreaError::BadSeparator) => try_different_separator(),
        _ => give_up(),
    }
}
```

<Nr />

