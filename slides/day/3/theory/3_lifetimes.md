```yaml
layout: cover
class: text-center
```

# Lifetimes

book chapter 10.3

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-60 left-89 w-10"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-109 w-7"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-138.5 w-7"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-168.5 w-7"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-56 left-89 w-10"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-56 left-109 w-7"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-56 left-138.5 w-7"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-56 left-168.5 w-7"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-65.5 left-155.5 w-4"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# LT Annotations in Structs

```rust
struct StoringBorrowedData<'number, 'text> {
    n: &'number i32,
    s: &'text str,
}
```

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
```

# Rust Easy Mode<sup>TM</sup>

premature optimization is the root of all evil

```rust
fn longest(x: &String, y: &String) -> String {
    if x.len() > y.len() {
        x.clone()
    } else {
        y.clone()
    }
}
```

`Clone` is your friend!

<Nr />
