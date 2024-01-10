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

# What are references?

- basically, pointers with seat belts
- cannot be null
- guaranteed to point to valid memory

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Syntax

```rust
let x = 42;
let r: &i32 = &x;
let y: i32 = *r;
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-77 left-124 w-9"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-77 left-139 w-5"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-82 left-137.5 w-4"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 2
```

# Fixing the earlier example

```rust {1,7|2-3|all}
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

<div
    style="background-color: red"
    class="h-0.5 absolute top-57 left-114.5 w-3"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-87 left-131.5 w-3"
    v-click="[0,1]"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Mutable References

```rust
let mut x = 42;
let r = &mut x;
*r = 43;
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-77.5 left-111 w-9"
></div>

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

# Mutable references are exclusive

```rust
let mut x = 12;

let r1 = &mut x;
let r2 = &mut x; // error

*r1 = 13;
```

compiler says:

> cannot borrow `x` as mutable more than once at a time

---

```yaml
layout: center
class: text-center
transition: none
```

# Mutable references are exclusive

```rust {4}
let mut x = 12;

let r1 = &mut x;
let r2 = &mut x; // error

*r1 = 13;
```

compiler says:

> cannot borrow `x` as mutable more than once at a time

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Mutable references are exclusive

```rust {4}
let mut x = 12;

let r1 = &mut x;
let r2 = &x;     // error

*r1 = 13;
```

compiler says:

> cannot borrow `x` as immutable because it is also borrowed as mutable

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Non-Lexical Lifetimes

```rust {4,7-8}
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);
}
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
println!("{}", r); // error
```

compiler says:

> `s` does not live long enough

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
clicks: 4
```

# Bending the Rules

book chapter 15

This is advanced and not needed in most Rust programs.

```rust {0|1|2|4-7|all}
let r1 = Rc::new(RefCell::new(String::from("Hello")));
let r2 = Rc::clone(&r1);

// both owners can do interleaved mutations!
r1.borrow_mut().push_str(" Venus");
r2.borrow_mut().push_str(", Mars");
r1.borrow_mut().push_str(" and Jupiter!");

println!("{}", r1.borrow()); // -> Hello Venus, Mars and Jupiter!
```

`Rc` : reference counting\
`RefCell` : interior mutability

<div
    style="background-color: red"
    class="h-0.5 absolute top-76.7 left-59 w-5"
    v-click="[3,4]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-81.8 left-59 w-5"
    v-click="[3,4]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-86.7 left-59 w-5"
    v-click="[3,4]"
></div>
