```yaml
layout: cover
class: text-center
transition: slide-up
```

# The Slice Type

book chapter 4.3

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Slices in C ?

```c
void print_slice(int *start, size_t len) {
    for (size_t i = 0; i < len; i++) {
        printf("%d ", start[i]);
    }
}
void main() {
    int numbers[5] = {1, 2, 3, 4, 5};
    print_slice(numbers + 1, 3);  // 2 3 4
    print_slice(numbers + 3, 10); // 👻👻👻
}
```

Start pointer and length are disconnected,\
the compiler cannot reason about memory safety.\
➔ buffer overflow

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Rust Slices

```rust
fn print_int_list(list: &[i32]) {
    for elem in list {
        print!("{} ", elem);
    }
}
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    print_int_list(&numbers[1..4]); // 2 3 4
    print_int_list(&numbers[3..13]); // panic: index out of range
}
```

Rust slices store their length alongside the start pointer.\
The full length of a slice is guaranteed valid memory.\
➔ no buffer overflow

<div
    style="border-color: red"
    class="border-1 absolute top-45 left-112 w-12"
></div>
<div
    style="border-color: red"
    class="border-1 absolute top-81 left-103 w-27"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# The String Slice

```rust
let owned = String::from("Hello, world!");
let s: &str = &owned[3..9]; // "lo, wo"

// Range boundaries must be valid UTF-8 offsets!
let s: &str = &"😱"[1..];
```

computer says:

> byte index 1 is not a char boundary; it is inside '😱' (bytes 0..4) of `😱`

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Borrowing rules apply to slices

```rust
let mut owned = String::from("hello");
let s: &str = &owned[2..];
owned.pop(); // error: cannot borrow as mutable
println!("{}", s);
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# String Literals

```rust
let greeting: &str = "Hello, world!";
```

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Off-Topic: Vectors

needed for exercises

```rust
let v: Vec<f64> = Vec::new(); // create empty, elems of type f64

let mut v = vec![1, 2, 3]; // macro for "vector literals"

v.push(4);
assert_eq!(v.pop().unwrap(), 4); // `unwrap` because `pop` might return "nothing"
```
