```yaml
layout: cover
class: text-center
transition: slide-up
```

# Iterators

book chapter 13.2

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Processing a Series of Items

```rust
trait Iterator {
    type Item; // associated type (new syntax)
    fn next(&mut self) -> Option<Self::Item>;
}
```

For a type to be an iterator, it needs to...

- define the type of the items it iterates over.
- provide a method to get the next item in the series.

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Using an Iterator (badly)

```rust {2|3|4,11,5|4,11,6-8|4,11,9|4,11,10|all}
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    loop {
        let maybe_item = iter.next();
        if maybe_item.is_none() {
            break;
        }
        let item = maybe_item.unwrap();
        println!("next item: {}", item);
    }
}
```

---

```yaml
layout: center
class: text-center
transition: none
```

# Refactoring

<!-- monaco-diff is not wide enough by default (bug) -->
<!-- force sufficent width so all text is visible -->
<div style="width: 700px"></div>

```rust {monaco-diff}
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    loop {
        let maybe_item = iter.next();
        if maybe_item.is_none() {
            break;
        }
        let item = maybe_item.unwrap();
        println!("next item: {}", item);
    }
}
~~~
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    loop {

        let Some(item) = iter.next() else {
            break;
        };

        println!("next item: {}", item);
    }
}
```

---

```yaml
layout: center
class: text-center
transition: none
```

# Refactoring

<!-- monaco-diff is not wide enough by default (bug) -->
<!-- force sufficent width so all text is visible -->
<div style="width: 700px"></div>

```rust {monaco-diff}
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    loop {
        let Some(item) = iter.next() else {
            break;
        };


        println!("next item: {}", item);
    }
}
~~~
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    while let Some(item) = iter.next() {





        println!("next item: {}", item);
    }
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Finally Understanding `for`-Loops

<!-- monaco-diff is not wide enough by default (bug) -->
<!-- force sufficent width so all text is visible -->
<div style="width: 700px"></div>

```rust {monaco-diff}
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    while let Some(item) = iter.next() {





        println!("next item: {}", item);
    }
}
~~~
fn main() {
    let v = vec![1, 2, 3];

    for item in v {





        println!("next item: {}", item);
    }
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 2
```

# Interlude: Turbofish

check out https://turbo.fish -- it's fun

```rust {2-4|6,13-16|8-10,13-16}
fn main() {
    println!("{}", "00123".parse().unwrap()); // ❌
    // type annotations needed
    // consider specifying the generic argument: `::<F>`

    println!("{}", "00123".parse::<i32>().unwrap());

    // why the double colon? why not this:
    println!("{}", "00123".parse<i32>().unwrap());
    // => syntax ambiguity with comparison operators ¯\_(ツ)_/¯
}

// for reference, from the standard library:
impl str {
    fn parse<F: FromStr>(&self) -> Result<F, <F as FromStr>::Err>;
}
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-69 left-126 w-14"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-83.5 left-126 w-10"
    v-click="[2,3]"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Iterator Adaptors

demo

---

```yaml
layout: center
class: text-center
transition: slide-left
```

# Performance?

<div></div>

Iterators and their adaptors are heavily optimized.\
Sometimes, they are even faster than hand-coded loops.

Rule of Thumb:

Do not pick one over the other based on performance speculations.\
If performance really matters, you need to benchmark.
