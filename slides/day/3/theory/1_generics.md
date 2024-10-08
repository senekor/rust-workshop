```yaml
layout: cover
class: text-center
```

# Generics

book chapter 10.1

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
let num: i32 = Some(42).unwrap();
let s: &str = Some("hello").unwrap();
```

Can we write `unwrap` ourselves?

<Nr />

---

```yaml
layout: center
class: text-center
```

# The Problem: Duplication

```rust
fn my_unwrap_i32(maybe_int: Option<i32>) -> i32 {
    maybe_int.unwrap()
}

fn my_unwrap_i64(maybe_int: Option<i64>) -> i64 {
    maybe_int.unwrap()
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-99 w-7"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-154 w-7"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-176 w-7"
></div>

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-84 left-99 w-7"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-84 left-154 w-7"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-84 left-176 w-7"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

<img
    src="/void_star.png"
    style="height: 360px"
/>

<Nr />

---

```yaml
layout: center
class: text-center
clicks: 2
```

# The Solution: Generics

```rust {1-3|5-7|all}
fn my_unwrap<T>(maybe_int: Option<T>) -> T {
    maybe_int.unwrap()
}

// compiler copies my_unwrap for each type
my_unwrap(Some(42_i32));
my_unwrap(Some(42_i64));
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-103 w-7"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-157 w-3"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-60 left-174.5 w-3"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-90 left-118 w-7"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-95.5 left-118 w-7"
    v-click="[1,2]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
clicks: 3
```

# Generics in Structs

```rust {1-4|6-8,12|2-3,6,9-12|all}
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mix = Point { x: 1.0, y: 10 };
    //                           ^^
    // mismatched types: expected float
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-45 left-107.3 w-5.5"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-51 left-93.8 w-3"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-56.8 left-93.8 w-3"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-81 left-148.3 w-3"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-87 left-144 w-7"
    v-click="[1,2]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Multiple Generic Type Parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mix = Point { x: 1.0, y: 10 };
    // inferred type: Point<f64, i32>
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Generics in Enums

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Generics in Methods

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-80.5 left-104 w-6.5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-80.5 left-126.5 w-6.5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-86.5 left-141 w-5"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Performance?

<div></div>

Generics are resolved at compile time.

Generic code is essentially copy-pasted for every type parameter.

There is zero runtime cost to using generics.

(just like C++ templates)

<Nr />
