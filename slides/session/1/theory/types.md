```yaml
layout: cover
class: text-center
transition: slide-up
```

# Basic Types

book chapter 3.2

---

```yaml
layout: center
transition: slide-up
```

# Integer types

| length  | signed  | unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

---

```yaml
layout: center
transition: slide-up
```

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |
| with type suffix | `57_i64`      |

---

```yaml
layout: center
transition: slide-up
```

# Floating-point types

IEEE-754

```rust
let x = 2.0; // default: 64-bit
let y: f32 = 3.0; // 32-bit
```

---

```yaml
layout: center
transition: slide-up
```

# Arithmetic operators

```rust
let x = 1 + 2 - 3 * 4 / 5 % 6;
```

---

```yaml
layout: center
transition: slide-up
```

# Booleans

```rust
let x = true;
let y: bool = false;
```

---

```yaml
layout: center
transition: slide-up
```

# Characters

unicode, UTF-8, always 32-bit

```rust
// notice the single quotes
let x = 'a';
let y: char = '漢';
let heart_eyed_cat = '😻';
```

---

```yaml
layout: center
transition: slide-up
```

# Tuples

```rust {1|1,3|1,4|all}
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup; // destructuring
let a: i32 = tup.0;  // indexing
```

---

```yaml
layout: center
transition: slide-up
```

# The empty tuple

also known as "unit type"

```rust
let rusty_void: () = println!("printing doesn't return anything");
```

---

```yaml
layout: center
```

# Arrays

size known at compile time

```rust {1|2|3|all}
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // initialize all element with the same value
let x = a[0];   // indexing
```
