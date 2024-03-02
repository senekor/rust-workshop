```yaml
layout: cover
class: text-center
transition: slide-up
```

# Ownership

book chapter 4.1

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Memory Management

| approach           | properties                                              |        |
| :----------------- | :------------------------------------------------------ | :----- |
| manual             | fast & predictable, but unsafe                          | ✅❌   |
| garbage-collection | slow & unpredictable, but safe                          | ❌✅   |
| ownership          | fast, predictable, safe and <Orange>expressive</Orange> | ✅✅✅ |

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# C and C++

a short history of manual memory management

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

## Double free

```c
int *p = malloc(sizeof(int));
free(p);
free(p); // ⚠️
```

<div class="h-8"></div>

## Use after free

```c
int *p = malloc(sizeof(int));
free(p);
*p = 12; // ⚠️
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Implicit ownership in C

```c
some_t *foo(some_t *p);
```

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>Is the function going to free the pointer, or do I have to?</li>
    <li>Does the function only read from the pointer or does it write to it?</li>
    <li>Can the return value alias the argument?</li>
    <li>Where is the documentation?</li>
    <li>Please let there be documentation.</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />

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

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Rust

codify and enforce the rules of ownership

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Ownership Rules

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <ol>
      <li>Every value has exactly one owner.</li>
      <li>When the owner goes out of scope, the destructor is run.</li>
    </ol>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />

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

<Nr />

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

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Reading the error message

<style>
.slidev-code {
  --slidev-code-font-size: 14px;
}
</style>

```txt {all|1,5,13}
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type String, which doesn't implement the Copy trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from ...
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

<Nr />

---

```yaml
layout: center
transition: slide-up
```

# Reading the error message

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // error
```

- borrow of moved value: `s1`

- move occurs because `s1` has type `String`,\
  which does not implement the `Copy` trait

- help: consider cloning the value\
  if the performance cost is acceptable

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Scope and Destructors

demo

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Ownership and Functions

```rust {3-8,10}
// recall: Foo's destructor prints "drop!"

fn main() {
    let x = Foo;
    take_foo(x);
    println!("Hello, world!");
}
fn take_foo(arg: Foo) {
    // empty function body
}
```

What's the output of this program?

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 6
```

# Ownership is expressive

file handles, mutexes etc.\
ownership applies to all kinds of resources

```rust {0|1|1|2|4,5|4,6,7|all} {at: 0}
fn foo(m: &Mutex<i32>, random_choice: bool) -> Option<MutexGuard<i32>> {
    let guard: MutexGuard<i32> = m.lock().unwrap();

    if random_choice {
        Some(guard)
    } else {
        None
    }
}
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-61 left-55 w-35"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-61 left-155 w-57"
    v-click="[2,3]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-left
clicks: 4
```

# Limitations

```rust {1|2-3|6,8|7|all} {at: 0}
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

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-54.5 left-133 w-34"
    v-click="[0,1]"
></div>

<Nr />
