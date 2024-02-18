// manually calling next

fn main() {
    let v = vec![1, 2, 3];

    let mut iter = v.into_iter();

    println!("next item: {}", iter.next().unwrap());
    println!("next item: {}", iter.next().unwrap());
    println!("next item: {}", iter.next().unwrap());

    // crashes:
    println!("next item: {}", iter.next().unwrap());
}
