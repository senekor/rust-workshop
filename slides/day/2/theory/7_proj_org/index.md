```yaml
layout: cover
class: text-center
transition: slide-up
```

# Project Organization

book chapter 7

---

```yaml
layout: center
class: text-center
transition: slide-up
```

<img
    src="/proj_org.png"
    style="transform: scale(0.5,0.5)"
/>

---

```yaml
layout: center
transition: slide-up
```

# The Crate

- executable crate: `main.rs` -- library crate: `lib.rs`
- the basic compilation unit for `rustc`
- can comprise multiple `.rs` files
- semantically versioned

## Simplification: `main.rs` is the crate!

---

```yaml
layout: center
transition: slide-up
```

# Above the Crate: The Package

- `Cargo.toml`
- the basic build unit for `cargo`
- build scripts, e.g. for linking against C-libs
- tests against the public API of a crate

## Simplification: `Cargo.toml` is the package!

---

```yaml
layout: center
transition: slide-up
```

# Below the Crate: Modules

- Code organziation
- Visibility & Encapsulation
- What we'll focus on next!

---

```yaml
layout: center
class: text-center
transition: slide-up
```

<img
    src="/proj_org.png"
    style="transform: scale(0.5,0.5)"
/>

---

```yaml
layout: center
transition: slide-left
```

```
❯ cargo new foo ; tree foo
     Created binary (application) `foo` package
foo
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```

<div
    class="border-2 border-orange-400 absolute top-54 left-56 w-64 h-32"
></div>
<div class="text-orange-400 absolute top-54 left-38 w-64 h-32">
    Package
</div>

<Arrow
    x1="540" x2="420" y1="316" y2="316"
    class="text-lime-400"
></Arrow>
<div class="text-lime-400 absolute top-75.8 left-138 w-64 h-32">
    Crate
</div>

---
src: ./modules.md
---
