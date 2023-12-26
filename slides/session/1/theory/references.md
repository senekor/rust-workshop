```yaml
layout: cover
class: text-center
transition: slide-up
```

# References and Borrowing

book chapter 4.2

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# One Mutable Reference

TODO

```rust
let mut x = 42;
let y = &mut x;
let z = &x;
*y += 1;
```

<!--
    TODO touch on smart pointers here.
    
    - mutex: show type system expressiveness of mutex guard
-->
