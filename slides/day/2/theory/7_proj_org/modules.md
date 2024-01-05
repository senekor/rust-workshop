```yaml
layout: cover
class: text-center
transition: slide-up
```

# Modules & Visibility

book chapter 7

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Declaring a Module

demo

<!--
    Let's not spend too much time on modules.
    They are not necessary to achieve anything in Rust.
    (except visibility boundaries)
-->

---

```yaml
layout: center
class: text-center
transition: slide-up
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
    class="h-0.5 absolute top-75 left-103.5 w-6.5"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Nesting

```rust {2,3,5-12}
fn main() {
    garden::fruits::orange();
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
    class="h-0.5 absolute top-80 left-90 w-4"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-85 left-102.5 w-6"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
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
    class="h-0.5 absolute top-92 left-102.5 w-10.5"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-102 left-102.5 w-10.5"
    v-click="[2,3]"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
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

---

```yaml
layout: center
class: text-center
transition: slide-up
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

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Using External Packages

<!-- toml renders wierdly, ini is close enough -->
```ini
# Cargo.toml
[dependencies]
rand = "0.8.5"
```

```rust
fn main() {
    let rand_num: i32 = rand::random();
    println!("your lucky number: {}", rand_num);
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
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
    class="h-0.5 absolute top-60 left-121 w-6"
    v-click="[0,1]"
></div>

---

```yaml
layout: center
```

# Summary

- `main.rs` is the root of the crate

- Modules can be defined inline or in separate files.

- The module tree is traversed with `::`, `super` and `crate`.

- Items are private unless made `pub`-lic.

- Verbose module path specifiers are avoided with `use`.
