```yaml
layout: cover
class: text-center
transition: slide-up
```

# Project Organization

book chapter 7

<Nr />

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

<Nr />

---

```yaml
layout: center
transition: slide-up
```

# The Crate

- executable crate: `main.rs` -- library crate: `lib.rs`
- the basic compilation unit for `rustc`
- can comprise multiple `.rs` files

## Simplification: `main.rs` is the crate!

<Nr />

---

```yaml
layout: center
transition: slide-up
```

# Above the Crate: The Package

- `Cargo.toml`, the basic build unit for `cargo`
- build scripts, e.g. for linking against C-libs
- tests against the public API of a crate
- semantically versioned

## Simplification: `Cargo.toml` is the package!

<Nr />

---

```yaml
layout: center
transition: slide-up
```

# Below the Crate: Modules

- Code organziation
- Visibility & Encapsulation
- What we'll focus on next!

<Nr />

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

<Nr />

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
    class="border-2 border-orange-500 absolute top-56.5 left-60 w-60 h-26"
></div>
<div class="text-orange-500 absolute top-56 left-40 w-64 h-32">
    Package
</div>

<Arrow
    x1="540" x2="420" y1="313" y2="313"
    class="text-lime-500"
></Arrow>
<div class="text-lime-500 absolute top-75 left-138 w-64 h-32">
    Crate
</div>

<Nr />

---
src: ./modules.md
---
