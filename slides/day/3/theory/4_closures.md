```yaml
layout: cover
class: text-center
transition: slide-up
```

# Closures

book chapter 13.1

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# What is a Closure?

<div></div>

Closures are inspired by functional programming, where anonymous functions are often created\
at runtime and passed around as argumets to and return values from other functions.\
They are sometimes called _lambdas_ by other languages.

Unlike functions, closures can capture values from the scope in which they’re defined.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Basic Syntax

```rust {2|2,3|3,4|4,5|5,6|8,9|all}
fn main() {
        fn multiply(x: i32, y: i32) -> i32 { x * y }
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    let multiply = |x: i32, y: i32|        { x * y };
    let multiply = |x     , y     |        { x * y };
    let multiply = |x     , y     |          x * y  ;

    // most concise:
    let multiply = |x, y| x * y;
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Closures as Arguments

```rust
fn main() {
    let x = 3;

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    nums.retain(|elem| elem % x == 0);

    println!("remaining: {:?}", nums); // [3, 6, 9]
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-62 left-86 w-3.5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-82 left-132 w-3.5"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Mutating the Environment

```rust
fn main() {
    let mut nums = vec![1, 2, 3];

    let mut push_seven = || nums.push(7);

    for _ in 0..10 {
        push_seven();
    }

    println!("nums: {:?}", nums);
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Forcing a Move

actually a copy in this case

```rust
fn main() {
    let x_squared;
    {
        let x = 3;
        x_squared = || x * x; // `x` does not live long enough
        x_squared = move || x * x; // ✅
    }
    println!("{}", x_squared());
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-86 left-104 w-9"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# The `Fn`-Traits

You won't have to use these traits directly,\
but you might see them in documentation and error messages.

|    Trait | Informal Meaning                             | Connection to Ownership Rules       |
| -------: | :------------------------------------------- | :---------------------------------- |
| `FnOnce` | can be called only once                      | moves captured value out of closure |
|  `FnMut` | can be called many times but not shared      | mutates captured value              |
|     `Fn` | can be called and shared without restriction | captures only immutable references  |

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# `Fn`-Trait Example

```rust
// Signature of `std::vec::Vec::retain`
// (T is the type of the elements of the vector)
//
pub fn retain<F>(&mut self, f: F)
where
    F: FnMut(&T) -> bool,
```

`retain` must call the function `f` multiple times (once per element).\
Therefore, the trait bound `FnOnce` would _not_ be enough.\
There is no reason to restrict mutation in `f`, so `FnMut` is the best choice.\
`Fn` would be unnecessarily restrictive.
