```yaml
layout: cover
class: text-center
transition: slide-up
```

# Recap of Day 3

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Generics

```rust {1-3|4-8|9-12|all}
fn my_unwrap<T>(maybe_val: Option<T>) -> T {
    maybe_val.unwrap()
}
impl<T> Option<T> {
    fn unwrap(self) -> T {
        // ...
    }
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Traits + Bounds

```rust {1,2,7|1,4-7|8|8-10|all}
trait Comparable {
    fn is_greater_than(&self, other: &Self) -> bool;

    fn is_less_than_or_equal(&self, other: &Self) -> bool {
        !self.is_greater_than(other)
    }
}
fn find_largest<T>(list: &[T]) -> &T
where
    T: Comparable,
{ /**/ }
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Lifetime Annotations

```rust
// possible
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}

// recommended
fn longest(x: &str, y: &str) -> String {}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Closures

```rust
fn main() {
    let x = 3;
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    nums.retain(|elem| elem % x == 0);
}
```

<Nr />

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Iterators

```rust {1-4|6|8-9|8-11|all}
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut iter = numbers.into_iter();
    while let Some(num) = iter.next() { /**/ }
    // equivalent:
    for num in numbers { /**/ }
}
```

<Nr />
