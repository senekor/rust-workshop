```yaml
layout: cover
class: text-center
```

# Pattern Matching

book chapter 6.2

<Nr />

---

```yaml
layout: center
class: text-center
```

# The `match` Expression

```rust {1-6|7|8-13|all}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Patterns that Bind to Values

```rust {1-6|2,8,9,17|3,8,10,17|4,8,11,17|5,8,12-16,17|all}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn receive(msg: Message) {
    match msg {
        Message::Quit => println!("bye bye!"),
        Message::Move { x, y } => set_position(x, y),
        Message::Write(text) => println!("{}", text),
        Message::ChangeColor(r, g, b) => {
            set_red(r);
            set_green(g);
            set_blue(b);
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

# Matching on `Option`

```rust {1-6,8-10}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Exhaustive Matching

```rust {2-5}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // forgot about None!
    }
}
```

compiler says:

> non-exhaustive patterns: `None` not covered

<Nr />

---

```yaml
layout: center
class: text-center
```

# Exhaustive Matching

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

# Catch-all Patterns

```rust
match 0_i32 {
    7  => println!("You are lucky! 🥳"),
    13 => println!("You are unlucky! 😢"),
    x  => println!("squared: {}", x * x),
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Catch-all Patterns

```rust {4}
match 0_i32 {
    7  => println!("You are lucky! 🥳"),
    13 => println!("You are unlucky! 😢"),
    x  => println!("squared: {}", x * x),
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Catch-all Patterns

```rust {4}
match 0_i32 {
    7  => println!("You are lucky! 🥳"),
    13 => println!("You are unlucky! 😢"),
    _  => {}
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Boilerplate...

```rust
match option_num {
    Some(num) => {
        println!("So much typing 🤧");
    }
    _ => {}
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Matching on a Single Pattern

```rust
let maybe_num: Option<i32> = Some(10);
if let Some(num) = maybe_num {
    println!("number detected: {}", num);
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-75 left-77 w-39"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Looping `while` a Pattern Matches

```rust
let mut numbers = vec![1, 2, 3];
while let Some(n) = numbers.pop() {
    println!("removed from vec: {}", n);
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-75 left-66 w-42"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Summary

enums and pattern matching

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>model alternatives in your data</li>
    <li>prevent invalid data access bugs</li>
    <li>branch over data structures with <code>match</code></li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />
