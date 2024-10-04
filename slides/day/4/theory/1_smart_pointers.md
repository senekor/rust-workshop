```yaml
layout: cover
class: text-center
```

# Smart Pointers

book chapter 15

<Nr />

---

```yaml
layout: center
class: text-center
```

# What is a Smart Pointer?

<div></div>

Smart pointers are data structures that act like a pointer\
but also have additional metadata and capabilities.

We've already seen them! `Vec` and `String` are smart pointers.\
They store their length and capacity as metadata.\
They have the capability to grow and shrink.

<Nr />

---

```yaml
layout: center
class: text-center
```

What does the smart pointer point to?

```rust
trait Deref {
    type Target;

    fn deref(&self) -> &Self::Target;
}
```

What happens when I'm done with the smart pointer?

```rust
trait Drop {
    fn drop(&mut self);
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Box

```rust
// pseudo-code for illustration

pub struct Box<T>(*mut T);

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        std::alloc::dealloc(self.0);
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# TODO

<Nr />
