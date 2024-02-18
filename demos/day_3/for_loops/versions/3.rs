// handling the None-case

fn main() {
    let v = vec!['a', 'b', 'c'];

    let mut iter = v.into_iter();

    loop {
        let item: Option<char> = iter.next();
        if item.is_none() {
            break;
        }
        let item: char = item.unwrap();
        println!("next item: {}", item);
    }
}
