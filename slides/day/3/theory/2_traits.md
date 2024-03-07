```yaml
layout: cover
class: text-center
transition: slide-up
```

# Traits

book chapter 10.2

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
transition: slide-up
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
transition: slide-up
```

# Define Shared Behavior

```rust
trait Comparable {
    fn is_greater_than(&self, other: &Self) -> bool;
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-72 left-73 w-10.5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-77.5 left-179.5 w-3"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
    class="h-0.8 rounded absolute top-54.5 left-98 w-14.5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-79.5 left-98 w-14.5"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
    class="h-0.8 rounded absolute top-52.5 left-97.5 w-27"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
transition: slide-up
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
    class="h-0.8 rounded absolute top-68 left-107 w-37"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
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
transition: slide-up
```

# Blanket Implementations

```rust
trait ToAngryString {
    fn to_angry_string(&self) -> String;
}

impl<T: ToString> ToAngryString for T {
    fn to_angry_string(&self) -> String {
        self.to_string().to_uppercase()
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-left
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
