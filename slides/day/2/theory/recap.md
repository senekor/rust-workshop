```yaml
layout: cover
class: text-center
transition: slide-up
```

# Recap of Day 1

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Variables

```rust {1|2|3|5|6}
let x = 5;
let mut x = 5;
let x: i32 = 5;

const MAGIC_NUMBER: i32 = 42;
static FAST_DATA: &str = "Performante Daten"
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Types

```rust {1|2|3|4|5|6}
let b: u8 = b'A';
let l: usize = "some string".len();
let c: char = '🦀';
let x: (u8, char) = (b'A', '🦀');
let void: () = ();
let arr: [i32; 5] = [100; 5];
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Functions

```rust
fn identity(x: i32) -> i32 { x }
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Control Flow

```rust {1|2-8|9|10-12}
let abs = if x < 0 { -x } else { x };
loop {
    if done() {
        break;
    } else {
        continue;
    }
}
while true {}
for elem in [10, 20, 30] {
    println!("the number is: {}", elem);
}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Ownership Rules

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <ol>
      <li>Every value has exactly one owner.</li>
      <li>When the owner goes out of scope, the destructor is run.</li>
    </ol>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Borrrowing Rules

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <ol>
      <li>ONE mutable reference OR many immutable ones.</li>
      <li>References always point to valid memory.</li>
    </ol>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<div class="h-4"></div>

```rust {0|1|2}
let read_only_ref: &String = &owned_string;
let mutable_ref: &mut String = &mut owned_string;
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Slices

```rust {1|2|3}
let byte_slice: &[u8] = &[1, 2, 3, 4, 5];
let middle_part = &byte_slice[1..4];
let string_slice: &str = "valid UTF-8";
```

<Nr />

