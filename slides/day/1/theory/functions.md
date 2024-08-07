```yaml
layout: cover
class: text-center
```

# Functions

book chapter 3.3

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-60 left-125 w-25"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-90 left-127 w-15"
    v-click="[1,2]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
clicks: 2
```

# Return Values

```rust {1,6|1,5,6|2-4}
fn plus_one(x: i32) -> i32 {
    if x == i32::MAX {
        return i32::MAX; // no overflow
    }
    x + 1  // <- last expression of block
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-63 left-126 w-15"
    v-click="[0,1]"
></div>

<Nr />
