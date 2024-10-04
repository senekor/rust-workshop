```yaml
layout: cover
class: text-center
```

# Closures

book chapter 13.1

<Nr />

---

```yaml
layout: center
class: text-center
```

# What is a Closure?

<div></div>

Closures are inspired by functional programming, where anonymous functions are often created\
at runtime and passed around as argumets to and return values from other functions.\
They are sometimes called _lambdas_ by other languages.

Unlike regular functions, closures can capture values from the scope in which they were defined.

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-84 left-133 w-3.5"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-88 left-100 w-10"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# What is the Type of a Closure?

```rust
let square: ??? = |x| x * x;
```

We don't need to write the type, but can we?

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-72 left-103 w-7"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# What is the Type of a Closure?

```rust
let square: fn(i32) -> i32 = |x| x * x;
```

This is a _function pointer_ and occupies space in memory.

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-72 left-103 w-34"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Doesn't work with capturing...

```rust
let x = 3;
let times_x: fn(i32) -> i32 = |y| x * y;
// ❌ error: expected fn pointer, found closure
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Another thing that doesn't work

```rust
let x = 3;
let mut times_x = |y| x * y;

let x = 5;
times_x = |y| x * y;
```

<div style="height: 1em"></div>

```txt {lines:false}
mismatched types
expected closure, found a different closure
  = note: no two closures, even if identical, have the same type
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# The `Fn`-Traits

|    Trait | Informal Meaning                             | Connection to Ownership Rules      |
| -------: | :------------------------------------------: | :--------------------------------- |
|     `Fn` | can be called and shared without restriction | captures only immutable references |
|  `FnMut` | can be called many times but not shared      | mutates captured values            |
| `FnOnce` | can be called only once                      | moves captured values into closure |

<div style="height: 1em"></div>

Closures have _unnamable_ types.\
We can only refer to them via the traits they implement.

<Nr />

---

```yaml
layout: center
class: text-center
```

# The `Fn`-Traits

```rust
let mut text_buffer = String::from("To whom it may concern\n\n");

// implements `Fn() -> usize`
let buf_len = || text_buffer.len();

// implements `FnMut(&str)`
let mut append_to_buf = |s| text_buffer.push_str(s);

// implements `FnOnce()`
let print_and_drop_buf = move || println!("{text_buffer}");
```

<div style="height: 1em"></div>

Note: `Fn` is a "superset" of `FnMut`, which in turn is a "superset" of `FnOnce`.

<Nr />

---

```yaml
layout: center
class: text-center
```

# `Fn`-Trait Example: `Vec::retain`

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
fn main() {
    let x = 3;

    let mut nums: Vec<_> = (1..=16).collect();

    my_retain(&mut nums, |elem| elem % x == 0);

    println!("remaining: {:?}", nums);
}

fn my_retain<P>(nums: &mut Vec<i32>, predicate: P)
where
    P: Fn(i32) -> bool,
{
    for i in (0..nums.len()).rev() {
        if !predicate(nums[i]) {
            nums.remove(i);
        }
    }
}
```

<Nr />
