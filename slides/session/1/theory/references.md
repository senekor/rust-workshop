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
transition: slide-up
```

# What is a reference?

- basically a pointer with a seat belt
- cannot be null
- guaranteed to point to valid memory

---

```yaml
layout: center
transition: slide-up
```

# Fixing the earlier example

```rust {1,7|1-4|all}
fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope, but its destructor is not run.
}
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Mutable References

demo

---

```yaml
layout: center
class: text-center
transition: none
```

# Mutable reference are exclusive

```rust {all|4}
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

computer says:

> cannot borrow `s` as mutable more than once at a time

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Mutable reference are exclusive

```rust {4}
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &s;

println!("{}, {}", r1, r2);
```

computer says:

> cannot borrow `s` as immutable because it is also borrowed as mutable

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Non-lexical lifetimes

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Dangling References

```rust
let r;
{
    let s = String::from("hello");
    r = &s;
}
println!("{}", r);
```

---

```yaml
layout: center
transition: slide-up
```

# Borrrowing Rules

- At any given time, you can have either one mutable reference\
  or any number of immutable references.

- References must always be valid.

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Bending the Rules

book chapter 15

This is advanced and not needed in most Rust programs.

```rust
let r1 = Rc::new(RefCell::new(String::from("Hello")));
let r2 = Rc::clone(&s);

// overlapping mutable borrows!
r1.borrow_mut().push_str(" Venus");
r2.borrow_mut().push_str(", Mars");
r1.borrow_mut().push_str(" and Jupiter!");

println!("{}", s.borrow()); // -> Hello Venus, Mars and Jupiter!
```

`Rc` : reference counting\
`RefCell` : interior mutability
