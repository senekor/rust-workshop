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
class: text-center
transition: slide-up
```

# Integer Types

| length  | signed  | unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Number Literals

|                  |               |
| ---------------: | :------------ |
|          Decimal | `98_222`      |
|              Hex | `0xff`        |
|            Octal | `0o77`        |
|           Binary | `0b1111_0000` |
|       ASCII Byte | `b'A'`        |
| with type suffix | `57_i64`      |

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Floating-point Types

IEEE-754

```rust
let x = 2.0; // default: 64-bit
let y: f32 = 3.0; // 32-bit
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Arithmetic Operators

```rust
let x = 1 + 2 - 3 * 4 / 5 % 6;
```

---

```yaml
layout: center
class: text-center
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
class: text-center
transition: slide-up
```

# Characters

unicode, guaranteed 32-bit

```rust
// notice the single quotes
let x = 'a';
let y: char = '漢';
let heart_eyed_cat = '😻';
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-63 left-137 w-12.5"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 2
```

# Tuples

```rust {1|3|4|all}
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup;
let a: i32 = tup.0;
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-84.5 left-119.5 w-4.5"
    v-click="[2,3]"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# The Empty Tuple

also known as the "unit"

```rust
let rusty_void: () = println!("printing doesn't return anything");
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-81.5 left-88 w-8"
></div>

---

```yaml
layout: center
class: text-center
clicks: 3
```

# Arrays

size known at compile time

```rust {1|2|3|all}
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // == [3, 3, 3, 3, 3]
let x = a[0];
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-76.5 left-105 w-14"
    v-click="[0,1]"
></div>
