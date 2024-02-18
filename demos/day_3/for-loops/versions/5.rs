// while-let

fn main() {
    let v = vec![1, 2, 3];

    let mut iter = v.into_iter();

    while let Some(item) = iter.next() {
        println!("next item: {}", item);
    }
}
