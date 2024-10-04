```yaml
layout: cover
class: text-center
```

# Iterators

book chapter 13.2

<Nr />

---

```yaml
layout: center
class: text-center
```

# Processing a Series of Items

```rust
trait Iterator {
    type Item; // associated type (new syntax)
    fn next(&mut self) -> Option<Self::Item>;
}
```

For a type to be an iterator, it needs to...

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>define the type of the items it iterates over.</li>
    <li>provide a method to get the next item in the series.</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<Nr />

---

```yaml
layout: center
class: text-center
```

````md magic-move
```rust {all|4|6|8-14|16-18|8-18}
// manually calling next

fn main() {
    let v = vec!['a', 'b'];

    let mut iter = v.into_iter();

    // a
    let item = iter.next().unwrap();
    println!("next item: {}", item);

    // b
    let item = iter.next().unwrap();
    println!("next item: {}", item);

    // crash
    let item = iter.next().unwrap();
    println!("next item: {}", item);
}
```
```rust {8-11|8-12}
// looping until crash

fn main() {
    let v = vec!['a', 'b'];

    let mut iter = v.into_iter();

    loop {
        let item = iter.next().unwrap();
        println!("next item: {}", item);
    }
    // still crashes at third iteration
}
```
```rust {9-13}
// handling the None-case

fn main() {
    let v = vec!['a', 'b'];

    let mut iter = v.into_iter();

    loop {
        let item: Option<char> = iter.next();
        if item.is_none() {
            break;
        }
        let item: char = item.unwrap();
        println!("next item: {}", item);
    }
}
```
```rust {9-11|8-11,13}
// let-else pattern matching

fn main() {
    let v = vec!['a', 'b'];

    let mut iter = v.into_iter();

    loop {
        let Some(item) = iter.next() else {
            break;
        };
        println!("next item: {}", item);
    }
}
```
```rust {8,10|6,8}
// while-let

fn main() {
    let v = vec!['a', 'b'];

    let mut iter = v.into_iter();

    while let Some(item) = iter.next() {
        println!("next item: {}", item);
    }
}
```
```rust {6}
// for-loop

fn main() {
    let v = vec!['a', 'b'];

    for item in v {
        println!("next item: {}", item);
    }
}
```
````

<Nr />

---

```yaml
layout: two-cols
class: text-center
```

# Before

```rust {4-11,13}{lines:false}
fn main() {
    let v = vec!['a', 'b'];

    let mut iter = v.into_iter();

    loop {
        let item = iter.next();
        if item.is_none() {
            break;
        }
        let item: char = item.unwrap();
        println!("next item: {}", item);
    }
}
```

::right::

# After

```rust {4-11,13}{lines:false}
fn main() {
    let v = vec!['a', 'b'];



    for item in v {





        println!("next item: {}", item);
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Many Ways to Iterate

```rust {1|3-5|7-11|13-17|all}
let nums = vec![1, 2, 3];

// take ownership of items, destroy collection.
let iter = nums.into_iter();
for elem in nums {}

// iterate over immutable references, leave collection intact.
let iter = nums.iter();
let iter = (&nums).into_iter();
for elem in nums.iter() {}
for elem in &nums {}

// iterate over mutable references, modify collection.
let iter = nums.iter_mut();
let iter = (&mut nums).into_iter();
for elem in nums.iter_mut() {}
for elem in &mut nums {}
```

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-67 left-126 w-16"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-85 left-126 w-11"
    v-click="[2,3]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Iterator Adapters

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

# Performance?

<div></div>

Iterators and their adapters are heavily optimized.\
Sometimes, they are even faster than hand-coded loops.

Rule of Thumb:

Do not pick one over the other based on performance speculations.\
If performance really matters, you need to benchmark.

<Nr />
