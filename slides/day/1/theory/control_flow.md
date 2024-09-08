```yaml
layout: cover
class: text-center
```

# Control Flow

book chapter 3.5

<Nr />

---

```yaml
layout: center
class: text-center
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

The variable `size` will hold one of the three strings.

<Nr />

---

```yaml
layout: center
class: text-center
```

# `loop`

`continue` and `break` work as expected

```rust
loop {
    println!("computer go brrr");
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# `while` Loops

```rust {3,7}
let mut countdown = 10;

while countdown != 0 {
    println!("{}!", countdown);

    countdown -= 1;
}

println!("LIFTOFF!!!");
```

<Nr />

---

```yaml
layout: center
class: text-center
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
    class="h-0.8 rounded absolute top-70 left-97 w-26"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-76 left-120 w-10"
    v-click="[1,2]"
></div>

<Nr />

---

```yaml
layout: center
class: text-center
```

# Some Operators

some assignment variants exist ( `+=` )

| Comparison | `==` `!=` `<` `<=` `>` `>=`      |
| ---------: | :------------------------------- |
| Arithmetic | `+` `-` `*` `/` `%`              |
| Boolean    | `&&` `\|\|` `!`                  |
| Bitwise    | `&` `\|` `^` `!` (no tilde!)     |
| Range      | `..` `..=` (integers and `char`) |

<Nr />

---

```yaml
layout: center
class: text-center
clicks: 1
```

# Integer Conversions

`as` exists, it but has some footguns

```rust {1,2|4,5}
// infallible
let x: i32 = 42_i16.into();

// fallible
let x: u32 = 42_i64.try_into().unwrap();
```

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-76 left-95 w-7.5"
    v-click="[0,1]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-76 left-118 w-7.5"
    v-click="[0,1]"
></div>

<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-94 left-95 w-7.5"
    v-click="[1,2]"
></div>
<div
    style="background-color: red"
    class="h-0.8 rounded absolute top-94 left-118 w-7.5"
    v-click="[1,2]"
></div>

<Nr />
