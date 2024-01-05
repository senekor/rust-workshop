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
    style="border-color: red"
    class="border-1 absolute top-81.5 left-103 w-9"
    v-click="[1,2]"
></div>

---

```yaml
layout: center
class: text-center
transition: none
```

# Mutability

<div style="width: 20vw">
```rust
let x = 5;
x = 6; // error: cannot assign twice to immutable variable `x`
```
</div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Mutability

<div style="width: 20vw">
```rust
let mut x = 5;
x = 6; // ✅
```
</div>

<div
    style="border-color: red"
    class="border-1 absolute top-75 left-77.5 w-5.5"
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
    style="border-color: red"
    class="border-1 absolute top-72.5 left-63 w-8"
    v-click="[0,1]"
></div>

<div
    style="border-color: red"
    class="border-1 absolute top-86 left-63 w-10"
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
    style="border-color: red"
    class="border-1 absolute top-61.5 left-115.5 w-2.5"
    v-click="[0,1]"
></div>
<div
    style="border-color: red"
    class="border-1 absolute top-70.2 left-122.6 w-2.5"
    v-click="[0,1]"
></div>

<div
    style="border-color: red"
    class="border-1 absolute top-61.5 left-115.5 w-11"
    v-click="[1,2]"
></div>
<div
    style="border-color: red"
    class="border-1 absolute top-93 left-115.5 w-13"
    v-click="[1,2]"
></div>
