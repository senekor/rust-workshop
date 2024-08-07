```yaml
layout: cover
class: text-center
```

# Interlude: Macros

<Nr />

---

```yaml
layout: center
class: text-center
```

# General 

<div></div>

Rust macros are a meta-programming feature like the C preprocessor.\
Unlike in C, macros operate on _tokens_ instead of text.

Rust macros use very specific syntax, so you can identify them easily.

<Nr />

---

```yaml
layout: center
class: text-center
```

# "function-like" macros 

```rust {all} /!/
let name = "Joe";
let age = 36;
println!("My friend {} is {} years old.", name, age);
```

These macros are identified by the exclamation mark.

The tokens within the parentheses are the inputs to the macro.\
(string literal, comma, identifier, comma, identifier)

The output of the macro is the actual code\
necessary to print the formatted string.

<Nr />

---

```yaml
layout: center
class: text-center
```

# "attribute-like" macros 

```rust {all} /!/
#[my_attribute_macro]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
const PI: usize = 3 // close enough
```

These macros are identified by the `#[]` syntax.

The tokens of the item immediately after the macro are its input.\
That includes the entire function definition of `add`,\
but **NOT** the definition of `PI`.

`my_attribute_macro` might output additional code that's related\
to `add` or even modify the function itself.\
However, it cannot generate code based on `PI`,\
since it doesn't know about it.

<Nr />
