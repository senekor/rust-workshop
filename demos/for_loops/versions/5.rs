// while-let

fn main() {
    let v = vec!['a', 'b', 'c'];

    let mut iter = v.into_iter();

    while let Some(item) = iter.next() {
        println!("next item: {}", item);
    }
}
