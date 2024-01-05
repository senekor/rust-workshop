```yaml
layout: cover
class: text-center
transition: slide-up
```

# Error Handling

book chapter 9

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Quick and Dirty

stack unwinding

```rust {2,3|4,5|6|7}
fn main() {
    let v = vec![1, 2, 3];
    let x: i32 = v[100]; // <- panic
    let x: Option<i32> = None;
    let x: i32 = x.unwrap(); // <- panic
    panic!("custom panic message");
    todo!("shush compiler, I'm not done");
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# The `Result` Enum

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

recall the `Option` type:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Error Handling

demo

<!--
    - unwrap -> expect
    - panic -> Option + `?`
    - Option -> Result<_, String>
    - Err(String) -> Err(AreaError)
    - replace ',' with 'x' in main
    - strip '-' in main
-->
