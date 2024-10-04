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

What does a smart pointer point to?

```rust
trait Deref {
    type Target;

    fn deref(&self) -> &Self::Target;
}
```

What happens when I'm done with a smart pointer?

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

# Recursive Types

```rust
enum List {
    Node(i32, List), // ❌ error: infinite size
    End,
}

enum List {
    Node(i32, Box<List>),
    End,
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Using the `Deref` trait

```rust
fn main() {
    let boxed_int = Box::new(5);
    let reference: &i32 = &boxed_int;
    let copy: i32 = *boxed_int;
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Shared Ownership

```rust
enum List {
    Node(i32, Rc<List>),
    End,
}
use List::*;

fn main() {
    let shared_node = Rc::new(Node(12, Rc::new(End)));
    // ref count: 1
    {
        let other_node = Node(24, Rc::clone(&shared_node));
        // ref count: 2
    }
    // ref count: 1
} // ref count: 0 (shared_node gets dropped)
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Interior Mutability

```rust
struct SecretAgent {
    collected_intel: RefCell<Option<&'static str>>,
}
impl SecretAgent {
    /// This is safe because `self` is immutable 😇
    fn tell_secret(&self, info: &'static str) {
        *self.collected_intel.borrow_mut() = Some(info); // muahaha I lied 😈
    }
}
fn main() {
    let trustworthy_person = SecretAgent { collected_intel: RefCell::new(None) };

    // safe because `tell_secret` takes an immutable reference (right?)
    trustworthy_person.tell_secret("Stacey cheated on her boyfriend 🤫");
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Violating Borrowing Rules at Runtime

```rust
fn main() {
    let x = RefCell::new(1);
    let r1 = x.borrow_mut();
    let r2 = x.borrow(); // panic! 😱 (instead of compiler error)
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Shared Ownership + Interior Mutability

```rust
fn main() {
    let x: Rc<RefCell<i32>> = Rc::new(RefCell::new(1));

    let r1 = Rc::clone(&x);
    let r2 = Rc::clone(&x);

    *r1.borrow_mut() += 1;
    *r2.borrow_mut() += 1;

    println!("{}", x.borrow()); // 3
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Reference Cycle == Memory Leak

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
struct PrintWhenDropped(char);
impl Drop for PrintWhenDropped {
    fn drop(&mut self) {
        println!("drop called on {}!", self.0) }
}
struct GraphNode {
    value: PrintWhenDropped,
    neighbor: Option<Rc<RefCell<GraphNode>>>,
}
fn main() {
    let a = Rc::new(RefCell::new(GraphNode {
        value: PrintWhenDropped('a'),
        neighbor: None,
    }));
    let b = Rc::new(RefCell::new(GraphNode {
        value: PrintWhenDropped('b'),
        neighbor: Some(Rc::clone(&a)),
    }));
    a.borrow_mut().neighbor.replace(Rc::clone(&b)); // reference cycle
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Summary

smart pointers

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>put data on the heap with <code>Box</code></li>
    <li>share ownership with <code>Rc</code></li>
    <li>allow interior mutability with <code>RefCell</code></li>
    <li>watch out for reference cycles 🙂</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />
