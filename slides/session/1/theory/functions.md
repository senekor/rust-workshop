```yaml
layout: cover
class: text-center
transition: slide-up
```

# Functions

book chapter 3.3

---

```yaml
layout: center
transition: slide-up
```

# Basic Function

```rust {9-11|4-6,9-11|all}
fn main() {
    println!("Hello, world!");

    // latest compiler technology:
    // functions can be used above their definition
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

---

```yaml
layout: center
transition: slide-up
```

# Parameters

type annotation mandatory

```rust {1|1,6}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn main() {
    print_labeled_measurement(5, 'h');
}
```

---

```yaml
layout: center
transition: slide-up
```

# Expressions

A block is an expression where the last expression\
of the block becomes the value of the entire block.

```rust
let y = {
    let x = 3;
    x + 1 // <- note the lacking semicolon
};
// y == 4
```

---

```yaml
layout: center
```

# Return Values

The `return` keyword is only needed for early return.

```rust {7|8|all}
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}"); // 6
}

fn plus_one(x: i32) -> i32 {
    x + 1  // <- last expression of block
}
```
