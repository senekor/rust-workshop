```yaml
layout: cover
class: text-center
```

# Dynamic Dispatch

book chapter 17.2

<Nr />

---

```yaml
layout: center
class: text-center
```

# Object-Oriented Programming

<div style="height: 1em"></div>

| **property** | **supported in Rust?** | **supporting feature** |
| :-: | :-: | :-: |
| associate data and behavior | ✅ | methods |
| encapsulation | ✅ | modules |
| polymorphism | ✅ | traits |
| inheritance | ❌ | |
| dynamic dispatch | ✅ | ❓ |

<Nr />

---

```yaml
layout: center
class: text-center
```

# Dynamic Dispatch

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
trait Animal {
    fn make_sound(&self);
}
struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("woof!") }
}
struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("meow!") }
}
fn get_animals() -> Vec<&'static dyn Animal> {
    vec![&Dog, &Dog, &Cat]
}
fn main() {
    for animal in get_animals() {
        animal.make_sound();
    }
}
```

<Nr />
