```yaml
layout: cover
class: text-center
```

# Basic Types

book chapter 3.2

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
```

# Floating-point Types

IEEE-754

```rust
let x = 2.0; // default: 64-bit
let y: f32 = 3.0; // 32-bit
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Booleans

```rust
let x = true;
let y: bool = false;
```

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-61 left-137 w-12.5"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Tuples

```rust {1}
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup;
let a: i32 = tup.0;
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Tuples

```rust {3}
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup;
let a: i32 = tup.0;
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Tuples

```rust {4} /.0/
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup;
let a: i32 = tup.0;
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# The Empty Tuple

also known as the "unit"

```rust
// [!code word:\: ()]
let rusty_void: () = println!("printing doesn't return anything");
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Arrays

size known at compile time

```rust {1} /[i32; 5]/
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // == [3, 3, 3, 3, 3]
let x = a[0];
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Arrays

size known at compile time

```rust {2}
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // == [3, 3, 3, 3, 3]
let x = a[0];
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Arrays

size known at compile time

```rust {3}
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // == [3, 3, 3, 3, 3]
let x = a[0];
```

<Nr />
