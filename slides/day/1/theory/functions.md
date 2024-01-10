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
class: text-center
transition: slide-up
```

# Basic Function

```rust {5-7|1-3}
fn main() {
    another_function();
}

fn another_function() {
    println!("Hello from another function!");
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 1
```

# Parameters

```rust {1|6}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn main() {
    print_labeled_measurement(5, 'h');
}
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-62.5 left-126 w-20"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.5 absolute top-87.5 left-127 w-14"
    v-click="[1,2]"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Expressions

A block is an expression where the last expression\
of the block becomes the value of the entire block.

```rust
let y = {
    let x = 3;
    x + 1  // <- note the lacking semicolon
};
// y == 4
```

---

```yaml
layout: center
class: text-center
clicks: 2
```

# Return Values

The `return` keyword is only needed for early return.

```rust {7|8|all}
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x); // 6
}

fn plus_one(x: i32) -> i32 {
    x + 1  // <- last expression of block
}
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-91.5 left-121 w-13.5"
    v-click="[0,1]"
></div>
