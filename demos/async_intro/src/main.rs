//
//
// Note that we need a dependency for the runtime.
//  vvvvv
use tokio::{runtime, time};

async fn do_stuff(name: &str) {
    println!("{name:>5}: He...");
    time::sleep(time::Duration::from_secs(1)).await;
    println!("{name:>5}: ...llo...");
    time::sleep(time::Duration::from_secs(1)).await;
    println!("{name:>5}: ...world!");
}

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();

    let alice_task = rt.spawn(do_stuff("Alice"));
    let bob_task = rt.spawn(do_stuff("Bob"));

    let both_tasks = async {
        alice_task.await.unwrap();
        bob_task.await.unwrap();
    };

    rt.block_on(both_tasks);
}
