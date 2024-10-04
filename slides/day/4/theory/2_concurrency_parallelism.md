```yaml
layout: cover
class: text-center
```

# Concurrency and Parallelism

book chapter 16

<Nr />

---

```yaml
layout: center
class: text-center
```

# Spawning Threads

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

# Message Passing

```rust {all|3|5-10|12-13|15-17|all}
fn main() {
    // mpsc: Multiple Producers, Single Consumer
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for message in ["hi", "from", "the", "thread"] {
            sender.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let message = receiver.recv().unwrap();
    println!("Got: {message}");

    for message in receiver {
        println!("Got: {message}");
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Shared-State Concurrency

<div></div>

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# `Send` and `Sync`

<div style="height: 1em"></div>

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>Types that can be sent (move ownership) across thread-boundaries are <code>Send</code>.</li>
    <li>
        Types where references to them can be sent across thread-boundaries are <code>Sync</code>.
        <br/>
        Intuitively, they can be read by multiple threads at the same time.
    </li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<div style="height: 2em"></div>

<div style="display: flex">
  <div style="flex-grow: 1"></div>
  <div style="text-align: left">
    <li>Most normal types are both <code>Send</code> and <code>Sync</code>.</li>
    <li><code>Rc</code> is <b>NEITHER</b> <code>Send</code> <b>NOR</b> <code>Sync</code>.</li>
    <li><code>RefCell</code> <b>IS</b> <code>Send</code>, but it is <b>NOT</b> <code>Sync</code>.</li>
    <li><code>Mutex</code> <b>IS</b> <code>Sync</code>, even if its contained type is only <code>Send</code>.</li>
  </div>
  <div style="flex-grow: 1"></div>
</div>

<div style="height: 1em"></div>

`Send` and `Sync` are <b>auto traits</b>, meaning the compiler<br/>
implements them for you where appropriate.

<Nr />

---

```yaml
layout: center
class: text-center
```

# Fearless Concurrency

<div style="height: 1em"></div>

With Rust, you can write concurrent programs<br/>
without having to be afraid of bugs like data races. 🥳<br/>

except...

### Deadlocks! 😢

<Nr />
