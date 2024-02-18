// manually calling next

fn main() {
    let v = vec!['a', 'b', 'c'];

    let mut iter = v.into_iter();

    // a
    let item = iter.next().unwrap();
    println!("next item: {}", item);

    // b
    let item = iter.next().unwrap();
    println!("next item: {}", item);

    // c
    let item = iter.next().unwrap();
    println!("next item: {}", item);

    // crash
    let item = iter.next().unwrap();
    println!("next item: {}", item);
}
