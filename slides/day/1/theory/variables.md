```yaml
layout: cover
class: text-center
```

# Variables

book chapter 3.1

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Variable Declaration

```rust {1}
let x = 5;

let x: i32 = 5;
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Variable Declaration

```rust {3} /: i32/
let x = 5;

let x: i32 = 5;
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Mutability

```rust {1-2}
let x = 5;
x = 6; // error: cannot assign twice to immutable variable `x`

let mut x = 5;
x = 6; // ✅
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Mutability

```rust {4-5}
let x = 5;
x = 6; // error: cannot assign twice to immutable variable `x`

// [!code word:mut:1]
let mut x = 5;
x = 6; // ✅
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Globals

```rust {1-2} /const/
// "copy-pasted" everywhere (like C's #define)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// placed in static memory (text or data segment)
static EMBEDDED_TEXT_FILE: &str = include_str!("path/to/some/file.txt");
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Globals

```rust {4-5}
// "copy-pasted" everywhere (like C's #define)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// placed in static memory (text or data segment)
// [!code word:static:1]
static EMBEDDED_TEXT_FILE: &str = include_str!("path/to/some/file.txt");
```

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
transition: none
```

# Shadowing

```rust {1-6}
let x: i32 = 5;
{
    let x = 6;
    // x == 6
}
// x == 5
let x: &str = "five";
```

<!-- Do not remove the shadowing slides - shadowing appears in rustlings -->

---

```yaml
layout: center
class: text-center
```

# Shadowing

```rust {1,7}
let x: i32 = 5;
{
    let x = 6;
    // x == 6
}
// x == 5
let x: &str = "five";
```
