```yaml
layout: cover
class: text-center
```

# Collections

book chapter 8

<Nr />

---

```yaml
layout: center
class: text-center
```

# Reading from Vectors Safely

```rust {2|4|6|7-10}
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    v[10]; // panics immediately! 😱

    let tenth: Option<&i32> = v.get(10); // safety: 💯
    match tenth {
        Some(tenth) => println!("The tenth element is {}", tenth),
        None => println!("There is no tenth element."),
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# `HashMap`

a.k.a. `dict`, `map`, hash table, associative array

```rust {1,4|6-7|9|11-13|all}
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Brazil", 1);
    scores.insert("Germany", 7);

    let germany_score: Option<&i32> = scores.get(&"Germany");
    
    for (key, value) in scores {
        println!("{} scored {} points!", key, value);
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# `std::collections::*`

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>Queue</li>
    <li>Linked-List</li>
    <li>Set</li>
    <li>Heap</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />
