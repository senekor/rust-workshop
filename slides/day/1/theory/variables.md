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
clicks: 1
```

# Variable Declaration

```rust {1|3}
let x = 5;

let x: i32 = 5;
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-82 left-105 w-10"
    v-click="[1,2]"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 1
```

# Mutability

```rust {1-2|4-5} {at:0}
let x = 5;
x = 6; // error: cannot assign twice to immutable variable `x`

let mut x = 5;
x = 6; // ✅
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-82 left-70.5 w-6.5"
    v-click="1"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 1
```

# Globals

```rust {1-2|4-5}
// "copy-pasted" everywhere (like C's #define)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// placed in static memory (text or data segment)
static EMBEDDED_TEXT_FILE: &str = include_str!("path/to/some/file.txt");
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-72.5 left-52 w-10.5"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.5 absolute top-87.3 left-52 w-12.5"
    v-click="[1,2]"
></div>

---

```yaml
layout: center
class: text-center
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
class: text-center
clicks: 1
```

# Shadowing

```rust {1-6|1,8}
let x: i32 = 5;
{
    let x = 6;
    // x == 6
}
// x == 5

let x: &str = "five";
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-59.5 left-113.5 w-3"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-69.5 left-122 w-3"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.5 absolute top-59.5 left-113.5 w-13"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-94.5 left-113.5 w-15"
    v-click="[1,2]"
></div>
