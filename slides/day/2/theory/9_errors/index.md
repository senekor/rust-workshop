```yaml
layout: cover
class: text-center
```

# Error Handling

book chapter 9

<Nr />

---

```yaml
layout: center
class: text-center
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
    todo!(); // shush compiler, I'm not done
}
```

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
```

# Error Handling

demo

<Nr />

<!--
    - unwrap -> expect
    - panic -> Option + `?`
    - Option -> Result<_, String>
    - Err(String) -> Err(AreaError)
    - replace ',' with 'x' in main
    - strip '-' in main
-->

---

```yaml
layout: center
class: text-center
```

```rust
#[derive(Debug)]
enum AreaError {
    BadSeparator,
    BadInteger(std::num::ParseIntError),
}
fn main() {
    match calculate_area("12x8") {
        Ok(area) => println!("area: {area}"),
        Err(e) => match e { /* compiler ensures all errors are handled */ }, }
}
fn calculate_area(input: &str) -> Result<usize, AreaError> {
    let (length, width) = match input.split_once('x') {
        Some(t) => t,
        None => return Err(AreaError::BadSeparator),
    };
    Ok(parse_number(length)? * parse_number(length)?)
}
fn parse_number(num: &str) -> Result<usize, AreaError> {
    match num.parse() {
        Ok(n) => Ok(n),
        Err(e) => Err(AreaError::BadInteger(e)), } }
```

<Nr />
