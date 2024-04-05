```
[[bench]]
name = "fib"
harness = false
```

```
fn main() {
    divan::main()
}
```

```
#[divan::bench()]
fn fib_rec() -> usize {
    fib::fib_rec(100)
}
```

```
#[divan::bench(args = [1,4,8,16,32])]
fn fib_rec(n: usize) -> usize {
    fib::fib_rec(n)
}
```
