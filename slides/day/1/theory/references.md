```yaml
layout: cover
class: text-center
```

# References and Borrowing

book chapter 4.2

<Nr />

---

```yaml
layout: center
```

# What are references?

- basically, pointers with seat belts
- cannot be null
- guaranteed to point to valid memory

<Nr />

---

```yaml
layout: center
class: text-center
```

# Syntax

```rust
let x = 42;
let r: &i32 = &x;
let y: i32 = *r;
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-77.5 left-124 w-10"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-77.5 left-141 w-5"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-83.5 left-138 w-4"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-54 left-112 w-3"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-90 left-132 w-3"
    v-click="[0,1]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Mutable References

```rust
let mut x = 42;
let r = &mut x;
*r = 43;
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-78 left-114 w-11"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Mutable References

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
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

<Nr />

---

```yaml
layout: center
class: text-center
```

# Borrrowing Rules

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <ol>
      <li>At any given time, you can have either one mutable reference<br/>
          or any number of immutable references.</li>
      <li>References must always be valid.</li>
    </ol>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />
