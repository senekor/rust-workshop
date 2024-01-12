```yaml
transition: slide-left
titleTemplate: "%s"
lineNumbers: true
fonts:
  mono: "Fira Mono"
class: text-center
```

# Rust Workshop

## Day 3

### Adrian Hornung

---

```yaml
layout: cover
```

# Agenda

<v-clicks>

- Chapter 10: Generic Types, Traits and Lifetimes
- Chapter 13: Functional Language Features: Iterators and Closures

</v-clicks>

---

# Generics: Example

Find the largest element in a list of i32.

<v-click>

```rust {all|2|3-8|10|all}
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

```

</v-click>

<!--

What if we want to find the largest element in a list of i32?
something like this:

1) get the first element
2) loop over the list, comparing the elements and
   storing the larger one
3) return the largest number

-->

---

# Generics: Example

Find the largest element in a list of char.

<v-click>

```rust {0-10|12-22|all} {lines:true}
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


```

</v-click>

<!--

known under many different names

looks exacly the same, right?

-->

---

# Generics: Example

```rust
fn largest<T>(list: &[T]) -> &T { ... }
```

<!--

Generics in rust use these <> (less then and greater then) symbols and to denote the "generic types"
-> looks like in java/ C++ with templates

Removing Duplication by Extracting

uses capital Letters (because it represents a struct)

-->

---

# Generics: Example

```rust {1-10|all}
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

<!--

let's try it out

to simulate the actuall call I have added a main function too

as you see it has a list

-->

---

# Generics: Error

```
$ cargo check
    Checking tmp v0.1.0 (/tmp/tmp)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `tmp` (bin "tmp") due to previous error

```

<!--

Obviously given the error message visible, we know that it didn't work.

look at the error message


it tells us we need something called a "std::cmp::PartialOrd" trait, if
we add that code it will run as we expect

We will talk about traits and this error again in a few minutes,
remeber this error for now.

-->

---

# Generics: in Struct and Impl

```rust
struct Pair<F, S> {
    f: F,
    s: S,
}

impl<F,S> Pair<F,S> {
    fn fst(&self) -> &F {
        &self.f
    }

    fn snd(&self) -> &S {
        &self.f
    }
}
```

<!--

a simple example of a struct of pairs of elements

-->

---

# Generics: Option / Result

```rust
enum Option<T> {
    Some(T)
    None,
}

impl<T> Option<T> {
    ...
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

impl<T, E> Result<T, E> {
    ...
}
```

<!--

you remeber the error handling portion from last time?
this is all the magic required to create those types

except for the compiler remending you that you need
to handle the cases.

-->

---

# Traits

- Shared Behavior
- Java: Interface
- C++: Abstract Classes

---

# Traits: Defining A Traits

```rust
struct Rectangle {
    side: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn circumference(&self) -> f64 {
        self.radius * self.radius
    }

    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}
```

<!--

Shared behaviour

let's do the traditional example of the Shapes

now how do we combine these?

-->

---

# Traits: Definition

```rust {1|2-3|5-6|all}
trait TraitName {
    // Required Function to be implemented
    fn requiredFunction(&self) -> u32;

    // function with a default implementation
    fn requiredFunctionTimeTwo(&self) -> u32 { self.requiredFunction() * 2 }
}
```

---

# Traits: Definition

```rust {1-3|1-3,9,19|all}
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    side: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}
```

<!--
make a trait
    => Shape

a single function
    => area(&self) -> f64
-->

---

# Traits: Definition #2

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    fn circumference(&self) -> f64 {
        self.radius * self.radius
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}
```

<!--

here is the missing part of the circle impl

-->
---

# Traits: In Vector

```rust {all|5}
// cut for brevity
fn main() {
    let rec = Rectangle { side: 42.0 };
    let cir = Circle { radius: 42.0 };
    let list = vec![ ? ];
    do_something(&list);
}

```

<!--
What can we use here to get  bot rec & cir into the list??
-->

---

# Traits: In Vector

```rust {4}
fn main() {
    let rec = Rectangle { side: 42.0 };
    let cir = Circle { radius: 42.0 };
    let list = vec![ rec, cir ];
    do_something(&list);
}
```

---

# Traits: In Vector

```shell
$ cargo check
    Checking tmp v0.1.0 (/tmp/tmp)
error[E0308]: mismatched types
  --> src/main.rs:29:27
   |
29 |     let list = vec![ rec, cir ];
   |                           ^^^ expected `Rectangle`, found `Circle`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `tmp` (bin "tmp") due to previous error

```

<!--
There is a way to to do this though.
-->

---

# Traits: In Vector

$\to$ Shared Behaviour!

```rust {4|all}
fn main() {
    let rec = Rectangle { side: 42.0 };
    let cir = Circle { radius: 42.0 };
    let list : Vec<Box<dyn Shape>> = vec![ Box::new(rec), Box::new(cir) ];
    do_something(&list);
}
```

<v-clicks>

- Box is a smart pointer $\Rightarrow$ Vec is using the generic size of a Box
  (so we are moving rec & cir onto the heap).
- Vec&lt;Box&lt;dyn Shape&gt;&gt; $\Rightarrow$ is the generic declaration of the vec
- dyn Shape $\Rightarrow$ means that we are using a dynamic object of type Shape

</v-clicks>

---

# Traits: Example Generics

```
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++
```

```rust {1|all}
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

<!--

remeber our previous example?

std::cmp:: is part of the preable so not needed here

std::cmp::PartialOrd

-->

---

# Traits: Complex Bounds

If we need to have more then one trait for a function

```rust
fn largest<T: PartialOrd + Debug>(list: &[T]) {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    println!("{:?}", largest);
}
```

---

# Traits: Complex Bounds

```rust
fn some_function<T: PartialEq + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32;
```

Equal but easier to read.

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: PartialEq + Clone
    U: Clone + Debug
```

<!--

Traits bounds can become very complicated fast

Arguably at that point you might have to overthink your design.

-->

---

# Traits: Derivable

Usefull for simple behaviour

- <Orange>Eq, PartialEq, Ord, PartialOrd</Orange>: to compare data types.
- <Orange>Clone</Orange>, to create T from &T via a copy.
- <Orange>Copy</Orange>, to give a type 'copy semantics' instead of 'move semantics'.
- <Orange>Hash</Orange>, to compute a hash from &T.
- <Orange>Default</Orange>, to create an empty instance of a data type.
- <Orange>Debug</Orange>, to format a value using the {:?} formatter.

<!--
A few default traits that you see basically every day in rust.
-->

---

# Traits: Derive

```rust {1-6|8-12}
struct Rectangle {
    side: f64
}

pub trait Clone {
    fn clone(&self) -> Self;
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        Self { side: self.side.clone() }
    }
}
```

<!--
We have seen PartialOrd, Clone and Debug

Now how do we use them?
Do we have to manually implement them?
-->

---

# Traits: Derive

```rust {1-6|8-12}
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Rectangle {
    side: usize
}
```

```rust
struct Rectangle {
    side: usize,
}
#[automatically_derived]
impl ::core::clone::Clone for Rectangle {
    #[inline]
    fn clone(&self) -> Rectangle {
        let _: ::core::clone::AssertParamIsClone<usize>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Rectangle {}
// cut for brevity (cut out about 60 more lines)
```

<!--
We can use this lovely feature to
automatically write the boilerplate code -- thank god

I checked out the ammount of code that we do not have to
write ourselves per struct
-->

---

# Traits: Drop

- Is automatically derived for you (you can manually implement).
- The function `drop` is always the last function called once the objects goes
  out of scope.
- Is usually used to automatically free up resources.

```rust
pub trait Drop {
    // Required method
    fn drop(&mut self);
}
```

<!--
The Drop trait only has one method: drop, which is called automatically when an
object goes out of scope. The main use of the Drop trait is to free the
resources that the implementor instance owns.
-->

---

# Lifetimes: What is it?

Remeber the scope of an object?

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

<!--

-->

---

# Lifetimes: Error

```shell
    Checking tmp v0.1.0 (/tmp/tmp)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let x = 5;
  |             - binding `x` declared here
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {}", r);
  |                       - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `tmp` (bin "tmp") due to previous error

```

---

# Lifetimes: The Borrow Checker

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}
```

A borrow my never life longer then the owner!

<!--

r has the lifetime 'a
x has the lifetime 'b


we see that the lifetime of 'b is shorter then 'a right?

that is one of the rules:
-> A borrow my never life longer then the owner!



ASK if this makes sense or if someone has questions here!!!!
-->

---

# Lifetime: Generics

```rust {1}
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<v-clicks>

```shell
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
          signature does not say whether it is borrowed from `x` or `y`
```

```shell
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
```

</v-clicks>

<!--
Problem that we are having is that the compiler doesn't know
for how long the return &str will live.

Rust never assumes anylive times if possible


Explain named life time parameter + similarities to generics.
=> every lifetime is unique -> so in theory there are as many
versions of this function as lifetimes it is used with.
(there only is a single version in reality, but for the
concept of the borrow checker it is good enough.)
-->

---

# Lifetime: Generics

```rust {1}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<!--

Just like the compiler asked - we add lifetime 'a everywhere
compiler is happy and so are we.
-->

---

# Lifetime: Generics

```rust {1-3|6-7|all}
struct Part<'buffer> {
    buffer: &'buffer [u8],
}

fn main() {
    let buf = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let p  = Part { buffer: &buf[..] };
    let p1 = Part { buffer: &buf[1..] };
    let p2 = Part { buffer: &buf[3..] };
}
```

<!--
Part allows us to safely operate on any buffer of bytes

=> Part can be replaced as offten as you like as long as
the compiler can guarantee that 'buffer lives longer.

Additionally I wanted to show you that for a lifetime as
well as a generic you can use full words.
-->

---

# Lifetime: Static

Special lifetime with $\infty$ timespan (basically the whole duration of a program).

```rust
let s: &'static str = "I have a static lifetime.";
```

<!--
You might see suggestions to use the 'static lifetime in error messages. But
honestly at that point you should rethink what you are doing, as it is rarely
the correct call.
-->

---

# Lifetimes: Generic Type + Trait Bound + Lifetimes

```rust {3|4-6|9|all}
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

# Closures

- Anonymous function that can capture their environment
- Basically function ptr and a prefilled struct it carries with itself.

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
```

```rust
struct Wrapper {
    callback: Box<dyn Fn(usize, usize) -> usize>,
}

```

---

# Closure: Traits

Special bounds (uses a bit of compiler magic for the trait syntax)

### FnOnce

Can be called <Orange>only once</Orange> and can be used a bound for all types of functions.

### FnMut

Can be called <Orange>more then once</Orange> and is <Orange>guaranteed to mutate</Orange> at least one captured
variable.

```rust
let mut x = 44;
let f  = |s: u32| { x += 2; s * x };
let b : Box<dyn FnMut(u32) -> u32> = Box::new(f);
```

### Fn

Can be called <Orange>more then once</Orange> and is <Orange>guaranteed not to mutate</Orange> any captured
variables.

```rust
let x = 44;
let f = |s: u32| { s * x };
```

<!--
Higher-Rank Trait Bounds (use a bit of compiler magic because of the syntax)
see https://doc.rust-lang.org/nomicon/hrtb.html

It is rare that you will have to care about these three traits, but in case
where you need to store a callback function. It is good to know.
-->

---

# Iterator

Allows to process a series of items

- Ranges (1..10) || (1..=10)
- Collections (Array, Vector, HashMap, Slices)
- Operation Chaining

```rust
pub trait Iterator {
    type Item;

    // required
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

<!--
I don't plan on explaining the type Item here.
But in essence it allows to abstract over a generic
return type Self::Item.
-->

---

# Iterators: Magic

It is not magic!

<v-click>

```rust {1-3|1-8|all}
for i in 0..10 {
    // ...
}

let mut s = 0..10;
while let Some(i) = s.next() {
    // ...
}

let mut s = 0..10;
loop {
    match s.next() {
        None -> break,
        Some(i) = {
            // ...
        }
    }
}
```

</v-click>

<!--
In some languages itarators are magical, not in rust
There is some syntax sugger, but only to an extend.
-->

---

# Iterators: Chaining

Say we want to sum up all the even numbers pow them to six from 1 until and
including 100.

$\sum^{100}_{i=1} i^6$

<v-clicks>

```rust
let mut s : u64 = 0;
for i in 1..=100u64 {
    if i % 2 == 0 {
        s += i.pow(6);
    }
}
```

```rust
let s : u64 = (1..=100u64)
    .filter(|e| *e % 2 == 0)
    .map(|e| e.pow(6))
    .sum(); // consumes the iterator above
```

</v-clicks>

<!--
It is a bit searched, but bear with me please. :)
-->

---

# Iterators: Enumerate

```rust
let v = vec!['a', 'b', 'c', ...];

for i in 0..(v.len()) {
    let val = v[i];
    // ...
}

for (i, val) in v.iter().enumerate() {
    // ...
}
```
