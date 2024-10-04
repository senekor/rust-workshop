```yaml
layout: cover
class: text-center
```

# Traits

book chapter 10.2

<Nr />

---

```yaml
layout: center
class: text-center
```

# What are Traits?

German: Eigenschaft, Merkmal

Traits fulfill the same purpose as interfaces in other languages.

They enable polymorphism by specifying shared behavior.

(Rust does not have OOP-style class inheritance.)

<Nr />

---

```yaml
layout: center
class: text-center
```

# Problem: `T` is useless

```rust {none|1|2|4,8|5-7|10|5|1,5|all}
fn find_largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

compiler says:

> binary operation `>` cannot be applied to type `&T`

<Nr />

---

```yaml
layout: center
class: text-center
```

# Define Shared Behavior

```rust
trait Comparable {
    fn is_greater_than(&self, other: &Self) -> bool;
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-72 left-63 w-12"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-78 left-189 w-3"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Implementation for Concrete Types

```rust
impl Comparable for i32 {
    fn is_greater_than(&self, other: &Self) -> bool {
        self > other
    }
}
impl Comparable for i64 {
    fn is_greater_than(&self, other: &Self) -> bool {
        self > other
    }
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-51 left-101 w-18"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-81 left-101 w-18"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Constrain Generic Type Parameters

```rust {1,5|all}
fn find_largest<T: Comparable>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item.is_greater_than(largest) {
            largest = item;
        }
    }

    largest
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-48 left-103 w-33"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Default Implementations

```rust
trait Comparable {
    fn is_greater_than(&self, other: &Self) -> bool;

    fn is_less_than_or_equal(&self, other: &Self) -> bool {
        !self.is_greater_than(other)
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Multiple Trait Bounds

```rust
fn find_largest<T: Comparable + Debug>(list: &[T]) -> &T {
    // ...
    println!("found {largest:?}!");
    largest
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-66 left-103 w-44"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Where Clauses

```rust
// hard to read
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// much better
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Blanket Implementations

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
use std::array;

fn main() {
    println!("{:?}", 42.clone_10_times());
    println!();
    println!("{:?}", String::from("hello").clone_10_times());
    println!();
    println!("{:?}", vec![1, 2].clone_10_times());
}

trait Clone10Times: Sized {
    fn clone_10_times(self) -> [Self; 10];
}

impl<T: Clone> Clone10Times for T {
    fn clone_10_times(self) -> [Self; 10] {
        array::from_fn(|_| self.clone())
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Useful Traits

|                      |                                        |
| -------------------: | :------------------------------------- |
|              `Debug` | string-representation for debugging    |
|     `Clone` & `Copy` | can be copied (cheaply)                |
|            `Default` | has default value (zero, empty string) |
|   `PartialEq` & `Eq` | can check for equality (`==`, `!=`)    |
| `PartialOrd` & `Ord` | can be ordered (`<`, `>` etc.)         |
|               `Hash` | can compute hash (for `HashMap` etc.)  |

**not derivable**: `Display`, `From` & `TryFrom`

<Nr />
