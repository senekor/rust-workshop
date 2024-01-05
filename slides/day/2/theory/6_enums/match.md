```yaml
layout: cover
class: text-center
transition: slide-up
```

# Pattern Matching

book chapter 6.2

---

```yaml
layout: center
class: text-center
transition: slide-up
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

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Patterns that Bind to Values

```rust {1-6|8,9,17|8,10,17|8,11,17|8,12-16,17}
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

---

```yaml
layout: center
class: text-center
transition: slide-up
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

---

```yaml
layout: center
class: text-center
transition: slide-up
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

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Exhaustive Matching

demo

---

```yaml
layout: center
class: text-center
transition: none
```

# Catch-all Patterns

```rust
match 0_i32 {
    7  => println!("You are lucky! 🥳"),
    13 => println!("You are unlucky! 😢"),
    x  => println!("squared: {}", x * x),
}
```

---

```yaml
layout: center
class: text-center
transition: none
```

# Catch-all Patterns

```rust {4}
match 0_i32 {
    7  => println!("You are lucky! 🥳"),
    13 => println!("You are unlucky! 😢"),
    x  => println!("squared: {}", x * x),
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Catch-all Patterns

```rust {4}
match 0_i32 {
    7  => println!("You are lucky! 🥳"),
    13 => println!("You are unlucky! 😢"),
    _  => {}
}
```

<!--
    if-let syntax is intentionally omitted,
    because clippy will mention it when a single-match pattern is used.
-->
<!--  -->

---

```yaml
layout: center
transition: slide-left
```

# Summary

enums and pattern matching

- model alternatives in your data

- prevent invalid data access bugs

- branch over data structures with `match`
