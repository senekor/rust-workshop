// handling the None-case

fn main() {
    let v = vec![1, 2, 3];

    let mut iter = v.into_iter();

    let maybe_item = iter.next();
    if maybe_item.is_some() {
        println!("next item: {}", maybe_item.unwrap());
    }
    let maybe_item = iter.next();
    if maybe_item.is_some() {
        println!("next item: {}", maybe_item.unwrap());
    }
    let maybe_item = iter.next();
    if maybe_item.is_some() {
        println!("next item: {}", maybe_item.unwrap());
    }

    // noop:
    let maybe_item = iter.next();
    if maybe_item.is_some() {
        println!("next item: {}", maybe_item.unwrap());
    }
}
