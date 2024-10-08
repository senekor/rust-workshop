```yaml
layout: cover
class: text-center
```

# Asynchronous Programming

book chapter still being worked on!

<Nr />

---

```yaml
layout: center
class: text-center
```

### Asynchronous Programming, or async for short,
### is a _concurrent programming model_.

<div style="height: 3em"></div>

### For practical purposes, it's an alternative to OS threads.

<Nr />

---

```yaml
layout: center
class: text-center
```

# Disadvantages of OS threads

<div style="height: 1em"></div>

A single thread has relatively large overhead, including its own stack.

Consequently, they are not well-suited for massive IO-bound workloads.<br/>
( e.g. high-traffic web servers )

Scheduling is done by the OS, implying the overhead of a context-switch.

Scheduling is preemptive, which is less efficient than cooperative.

<Nr />

---

```yaml
layout: center
class: text-center
```

# Async by comparison

<div style="height: 1em"></div>

Essentially zero overhead, not even heap allocations.

Perfectly suited for massive IO-bound workloads.

Scheduling is cooperative and works without context-switches.

<div style="height: 1em"></div>

...but more difficult to use!

<Nr />

---

```yaml
layout: center
class: text-center
```

# Async hello world

demo

<Nr />

---

```yaml
layout: center
class: text-center
```

```rust
use tokio::time;

async fn do_stuff(name: &str) {
    println!("{name:>5}: He...");
    time::sleep(time::Duration::from_secs(1)).await;
    println!("{name:>5}: ...llo...");
    time::sleep(time::Duration::from_secs(1)).await;
    println!("{name:>5}: ...world!");
}

#[tokio::main(worker_threads = 1)]
async fn main() {
    let alice_task = tokio::spawn(do_stuff("Alice"));
    do_stuff("Bob").await;
    alice_task.await.unwrap();
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Async & Embedded

```rust
use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
use embassy_time::{Duration, Timer};

// Declare async tasks
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}
```

<Nr />

---

```yaml
layout: center
class: text-center
```

# Recommended talk:
# Async Rust in Embedded Systems with Embassy

<div style="height: 2em"></div>
    
<div style="display: flex">
    <div style="flex-grow: 1"></div>
    <Youtube id="H7NtzyP9q8E" />
    <div style="flex-grow: 1"></div>
</div>

<Nr />
