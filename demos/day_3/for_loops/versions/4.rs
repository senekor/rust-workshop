// let-else pattern matching

fn main() {
    let v = vec!['a', 'b', 'c'];

    let mut iter = v.into_iter();

    loop {
        let Some(item) = iter.next() else {
            break;
        };
        println!("next item: {}", item);
    }
}
