// manual loop

fn main() {
    let v = vec![1, 2, 3];

    let mut iter = v.into_iter();

    loop {
        let maybe_item = iter.next();
        if maybe_item.is_none() {
            break;
        }
        let item = maybe_item.unwrap();
        println!("next item: {}", item);
    }
}
