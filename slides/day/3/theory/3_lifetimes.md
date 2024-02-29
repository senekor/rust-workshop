```yaml
layout: cover
class: text-center
transition: slide-up
```

# Lifetimes

book chapter 10.3

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Reminder:
# Rust forbids invalid references

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Problem: Returning References

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

compiler says:

> missing lifetime specifier:\
> this function's return type contains a borrowed value,\
> but the signature does not say whether it is borrowed from `x` or `y`

---

```yaml
layout: center
class: text-center
transition: none
```

# Solution: Lifetime Annotations

```rust {1}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-62.5 left-95 w-8"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-62.5 left-114 w-4"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-62.5 left-139.5 w-4"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-62.5 left-164.5 w-4"
></div>

---

```yaml
layout: center
class: text-center
transition: none
```

# Lifetime Annotations

```rust {1}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### There is some lifetime `'a`.

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-58 left-95 w-8"
></div>

---

```yaml
layout: center
class: text-center
transition: none
```

# Lifetime Annotations

```rust {1}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### `x` lives at least for `'a`.

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-58 left-114 w-4"
></div>

---

```yaml
layout: center
class: text-center
transition: none
```

# Lifetime Annotations

```rust {1}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### `y` lives at least for `'a`.

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-58 left-139.5 w-4"
></div>

---

```yaml
layout: center
class: text-center
transition: none
```

# Lifetime Annotations

```rust {1}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### The returned reference lives at least for `'a`.

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-58 left-164.5 w-4"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Lifetime Annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

In essence:\
The lifetime of the returned reference\
is the shorter one of `x` and `y`'s lifetimes.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Limitations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {/**/}

fn main() {
    let long;
    let x = String::from("looooong string");
    {
        let y = String::from("short str");
        long = longest(&x, &y);
    }
    println!("The longest string is: {}", long);
}
```

compiler says:

> `y` does not live long enough

...but we know this would be OK at runtime.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Alternatively...

```rust
fn maybe_strip_prefix<'a>(x: &'a str, y: &str) -> &'a str {
    x.strip_prefix(y).unwrap_or(x)
}
```

The lifetime of `y` has no relation to the lifetime of the return value.

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-66 left-152 w-4"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# LT Annotations in Structs

```rust
struct StoringBorrowedData<'number, 'text> {
    n: &'number i32,
    s: &'text str,
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Lifetime Elision Rules

(slightly simplified)

★ For a single parameter, its lifetime is assigned to all outputs.

```rust
fn foo(single_arg: &str) -> &str {}
// is the same as
fn foo<'a>(single_arg: &'a str) -> &'a str {}
```

★ For methods, the lifetime of `self` is assigned to all outputs.

```rust
impl Whatever {
    fn foo(&self, second_arg: &str) -> &str {}
    // is the same as
    fn foo<'a>(&'a self, second_arg: &str) -> &'a str {}
}
```

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# The `'static` Lifetime

```rust {1|4|6-11}
static GREETING: &'static str = "hello world";

fn main() {
    let greeting: &'static str = "hello world";

    let answer: &'static i32;
    {
        let heap_alloc = Box::new(42);
        answer = Box::leak(heap_alloc); // explicit memory-leak
    }
    println!("answer: {}", answer);
}
```
