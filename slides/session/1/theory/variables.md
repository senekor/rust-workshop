```yaml
layout: cover
class: text-center
transition: slide-up
```

# Variables

book chapter 3.1

---

```yaml
layout: center
transition: slide-up
```

# Variable declaration

```rust
let x = 5;
```

---

```yaml
layout: center
transition: slide-up
```

# Type annotation

usually inferred, sometimes necessary

```rust
// signed 32-bit integer
let x: i32 = 5;
```

---

```yaml
layout: center
transition: slide-up
```

# Mutability

```rust {1-3|5-7}
// error: cannot assign twice to immutable variable `x`
let x = 5;
x = 6;

// use keyword `mut` to make x mutable
let mut x = 5;
x = 6;
```

---

```yaml
layout: center
transition: slide-up
```

# Globals

type annotation mandatory

```rust {1-2|4-5}
// "copy-pasted" everywhere (like C's #define)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// placed in static memory (e.g. data segment)
static EMBEDDED_TEXT_FILE: &str = include_str!("path/to/some/file.txt");
```

---

```yaml
layout: center
transition: slide-up
```

# Scope

```rust
let x = 5;
{
    let y = 6;
    // x and y available
}
// only x available
```

---

```yaml
layout: center
```

# Shadowing

makes hungarian notation unnecessary

```rust {1-6|1,8}
let x = 5;
{
    let x = 6;
    // x == 6
}
// x == 5

let x: String = int_to_string(x);
// int_to_string doesn't actually exist
```
