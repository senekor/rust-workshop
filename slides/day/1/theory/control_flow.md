```yaml
layout: cover
class: text-center
transition: slide-up
```

# Control Flow

book chapter 3.5

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# `if` Expressions

no parentheses around condition, curly brackets mandatory

```rust
let number = 3;

let size = if number < 5 {
    "small"
} else if number < 10 {
    "big"
} else {
    "very big"
};
```

What is the type of `size` ?

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# `loop`

`continue` and `break` work as expected

```rust
loop {
    println!("computer go brrr");
}
```

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# `while` Loops

```rust {3,7}
let mut number = 10;

while number != 0 {
    println!("{}!", number);

    number -= 1;
}

println!("LIFTOFF!!!");
```

---

```yaml
layout: center
class: text-center
transition: slide-up
clicks: 3
```

# `for` Loops

more details on day 3

```rust {1,4,7|1,4-7|1,9,11|all}
let a = [10, 20, 30, 40, 50];

// `..` is the range operator
for i in 0..a.len() {
    let element = a[i];
    println!("the value is: {}", element);
}

for element in a {
    println!("the value is: {}", element);
}
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-71.8 left-102 w-22"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.5 absolute top-77 left-121.5 w-8"
    v-click="[1,2]"
></div>

---

```yaml
layout: center
class: text-center
transition: slide-up
```

# Off-Topic: Operators

some assignment variants exist ( `+=` )

| Comparison | `==` `!=` `<` `<=` `>` `>=`      |
| ---------: | :------------------------------- |
| Arithmetic | `+` `-` `*` `/` `%`              |
| Boolean    | `&&` `\|\|` `!`                  |
| Bitwise    | `&` `\|` `^` `!` (no tilde!)     |
| Range      | `..` `..=` (integers and `char`) |

---

```yaml
layout: center
class: text-center
transition: slide-left
clicks: 1
```

# Off-Topic: Integer Conversions

`as` exists, it but has some footguns

```rust {1,2|4,5}
// infallible
let x: i32 = 42_i16.into();

// fallible
let x: u32 = 42_i64.try_into().unwrap();
```

<div
    style="background-color: red"
    class="h-0.5 absolute top-76.5 left-89 w-5.5"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-76.5 left-108 w-5.5"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.5 absolute top-91.3 left-89 w-5.5"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.5 absolute top-91.3 left-108 w-5.5"
    v-click="[1,2]"
></div>
