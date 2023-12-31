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

---

```yaml
layout: center
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
transition: slide-up
```

# `while` Loops

```rust {3,7|all}
let mut number = 3;

while number != 0 {
    println!("{number}!");

    number -= 1;
}

println!("LIFTOFF!!!");
```

---

```yaml
layout: center
transition: slide-up
```

# `for` Loops

more details in session 3

```rust {1,4-7|1,9-11|all}
let a = [10, 20, 30, 40, 50];

// `..` is the range operator
for i in 0..a.len() {
    let element = a[i];
    println!("the value is: {element}");
}

for element in a {
    println!("the value is: {element}");
}
```

---

```yaml
layout: center
transition: slide-left
```

# Off-Topic: Operators

| Comparison | `==` `!=` `<` `<=` `>` `>=` |
| --- | --- |
| Arithmetic | `+` `-` `*` `/` `%` |
| Boolean | `&&` `\|\|` `!` |
| Bitwise | `&` `\|` `^` `!` (no tilde!) |
| Range | `..` `..=` (integers and `char`) |

assignment variants: `+=`, `|=`, `<<=` etc.
