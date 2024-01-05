```yaml
layout: cover
class: text-center
transition: slide-up
```

# Collections

book chapter 8

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Reading from Vectors Safely

```rust {2|4|6|7-10}
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    v.get[10]; // panics immediately! 😱

    let tenth: Option<&i32> = v.get(10); // safety: 💯
    match tenth {
        Some(tenth) => println!("The tenth element is {}", tenth),
        None => println!("There is no tenth element."),
    }
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# `HashMap`

a.k.a. `dict`, `map`, hash table

A collection of key-value pairs.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Brazil"), 1);
    scores.insert(String::from("Germany"), 7);
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Reading from a Hash Map

```rust {9-10}
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Brazil"), 1);
    scores.insert(String::from("Germany"), 7);

    let team_name = String::from("Germany");
    let germany_score = scores.get(&team_name).unwrap_or_default();
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Iterating over a Hash Map

```rust {9-11}
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Brazil"), 1);
    scores.insert(String::from("Germany"), 7);

    for (key, value) in scores {
        println!("{} scored {} points!", key, value);
    }
}
```
