// looping until crash

fn main() {
    let v = vec!['a', 'b', 'c'];

    let mut iter = v.into_iter();

    loop {
        let item = iter.next().unwrap();
        println!("next item: {}", item);
    }
    // still crashes at forth iteration
}
