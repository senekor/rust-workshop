```yaml
layout: cover
class: text-center
```

# Modules & Visibility

book chapter 7

<Nr />

---

```yaml
layout: center
class: text-center
```

# Declaring a Module

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
// main.rs - the root of the crate

mod inline_module { /* more code... */ }

mod file_module; // located in file_module.rs
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# The `pub` Keyword

```rust {2,3,5-8}
fn main() {
    garden::carrot();
    garden::lettuce(); // error
}
mod garden {
    pub fn carrot() {}
    fn lettuce() {}
}
```

compiler says:

> function `lettuce` is private

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-76.5 left-99.5 w-6.5"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Nesting

```rust {2,3,5-12}
fn main() {
    garden::fruits::orange();   // ✅
    garden::veggies::lettuce(); // error
}
mod garden {
    pub mod fruits {
        pub fn orange() {}
    }
    mod veggies {
        pub fn lettuce() {}
    }
}
```

compiler says:

> module `veggies` is private

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-82 left-85 w-5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-88.5 left-98 w-7"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
clicks: 3
```

# Paths

```rust {2-4,6-7|1-4,9,11-12,16-17|1-4,9,13-14,16-17|all} {at: 0}
mod garden {
    pub mod fruits {
        pub fn orange() {}
    }
    fn do_stuff() {
        // relative path
        fruits::orange();
    }
    mod veggies {
        fn lettuce() {
            // absolute path
            crate::garden::fruits::orange();
            // walking the module tree backwards
            super::fruits::orange()
        }
    }
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-96 left-98 w-12"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-108 left-98 w-12"
    v-click="[2,3]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Struct Field Visibility

```rust {2-5|8|3-4,10-11}
mod garden {
    pub struct Melon {
        pub size: u32,
        ripeness: u32,
    }
}
fn main() {
    let m: garden::Melon; // imagine initialization

    m.size; // ok
    m.ripeness; // error, field is private
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# The `use` Keyword

```rust {1-3,5|1-3,7-8}
mod garden {
    pub struct Melon;
}
fn main() {
    let m: garden::Melon;

    use garden::Melon;
    let m: Melon;
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Using External Packages

<!-- toml renders wierdly, ini is close enough -->
```ini
# Cargo.toml
[dependencies]
rand = "0.8.5"
```

<div class="h-2"></div>

```rust
fn main() {
    let rand_num: i32 = rand::random();
    println!("your lucky number: {}", rand_num);
}
```

<Nr />

---

```yaml
layout: center
class: text-center
clicks: 1
```

# Glob Imports

```rust {1-5|7-8} {at: 0}
use std::collections::*;
fn main() {
    let hm: HashMap;
    let bm: BTreeMap;
}

// common pattern for framework-like libraries:
use leptos::prelude::*;
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-58 left-122 w-6"
    v-click="[0,1]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
clicks: 3
```

# Re-Exporting with `pub use`

```rust {1-7|1-7,9|1-7,10-11|1-7,12-13}
mod garden {
    mod fruits {
        pub struct Melon;
        pub struct Orange;
    }
    pub use fruits::Melon;
}
fn main() {
    let m: garden::Melon;
    // doesn't work, because garden doesn't re-export Orange
    let o: garden::Orange;
    // doesn't work, because fruits is private
    let o: garden::fruits::Orange;
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-69 left-64 w-16"
    v-click="[0,1]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Renaming Imports with `as`

```rust
mod german {
    pub struct Kartoffel;
}
mod swiss_german {
    pub use super::german::Kartoffel as Herdoepfel;
}
fn main() {
    let same_thing: german::Kartoffel = swiss_german::Herdoepfel;
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-78 left-139 w-31"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Summary

modules and visibility

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li><code>main.rs</code> is the root of the crate.</li>
    <li>Modules can be defined inline or in separate files.</li>
    <li>The module tree is traversed with <code>::</code>, <code>super</code> and <code>crate</code>.</li>
    <li>Items are private unless made <code>pub</code>-lic.</li>
    <li>Verbose module path specifiers are avoided with <code>use</code>.</li>
    <li>Re-exporting allows fine-grained control over visibility.</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />
