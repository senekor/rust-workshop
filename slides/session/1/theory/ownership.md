```yaml
layout: cover
class: text-center
transition: slide-up
```

# Ownership

book chapter 4.1

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Approaches to memory management

|                    |                                                           |
| ------------------ | --------------------------------------------------------- |
| manual             | fast & predictable, but unsafe                            |
| garbage-collection | slow & unpredictable, but safe                            |
| ownership          | <Orange>fast, predictable, safe _and expressive_</Orange> |

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# C/C++

a short history of manual memory management

---

```yaml
layout: center
transition: slide-up
```

# Double `free`

```c
int *p = malloc(sizeof(int));
free(p);
free(p);
```

---

```yaml
layout: center
transition: slide-up
```

# Use after `free`

```c
int *p = malloc(sizeof(int));
free(p);
*p = 12;
```

---

```yaml
layout: center
transition: slide-up
```

# Whatever this is

```c
int *silly_foot_bazooka() {
    int i;
    return &i;
}
```

---

```yaml
layout: center
transition: slide-up
```

# Implicit ownership in C

```c
some_t *foo(some_t *p);
```

Is the function going to free the pointer, or do I have to?

Does the function only read from the pointer or does it write to it?

Can the return value alias the argument?

Where is the documentation? <Orange>Please</Orange> let there be documentation.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# C++

tools to express ownership

```cpp
std::unique_ptr<some_t> foo(some_t const* p);
```

and destructors!

...but no compiler guarantees.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Rust

codify and enforce the rules of ownership

---

```yaml
layout: center
transition: slide-up
```

# Ownership Rules

1. Every value has exactly one owner.
1. When the owner goes out of scope, the destructor is run.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Heap-allocated Strings

```rust
let embedded_in_binary: &str = "hello"; // immutable

{
    let mut heap_allocated: String = String::from("hello");
    heap_allocated.push_str(", world!");

} // drop(heap_allocated)
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# One Owner

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // error
```

---

```yaml
transition: slide-up
```

# Reading the error message

```txt
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

<div
    style="border-color: red"
    class="border-2 absolute top-34.5 left-49.5 w-51.5 h-5.5"
></div>
<div
    style="border-color: red"
    class="border-2 absolute top-52.5 left-51 w-157 h-5.5"
></div>
<div
    style="border-color: red"
    class="border-2 absolute top-88.5 left-24 w-130 h-5.5"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Stack-only Data: Copy

recall the error message:\
"move occurs because [String] does not implement the Copy trait"

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y); // this is fine!
```

stack-only types, like `i32`, can be copied cheaply and automatically

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Scope and Destructors

demo

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Ownership and Functions

```rust {2-7}
// recall: Foo has a destructor with a print statement
fn main() { // order of print statements?
    let x = Foo;
    take_foo(x);
    println!("Hello, world!")
}
fn take_foo(arg: Foo) {}
```

```rust
fn main() { // does this compile?
    let x = 42;
    take_i32(x);
    println!("The number is {x}")
}
fn take_i32(arg: i32) {}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Ownership is expressive

file handles, mutexes etc.\
ownership handles all kinds of resources

```rust {1|2|4,5|4,6,7|all}
fn foo(m: &Mutex<i32>, random_choice: bool) -> Option<MutexGuard<i32>> {
    let guard = m.lock().ok()?;

    if random_choice {
        Some(guard)
    } else {
        None
    }
}
```

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Limitations

```rust {1|2-3|6,8|7|all}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
```
