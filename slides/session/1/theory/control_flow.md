```yaml
layout: cover
class: text-center
transition: slide-up
```

# Control flow

book chapter 3.5

---

```yaml
layout: center
transition: slide-up
```

# `if` expressions

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

# `loop` expressions

```rust {3,7,9|all}
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

---

```yaml
layout: center
transition: slide-up
```

# `while` loops

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
```

# `for` loops

more details on day 3

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
