//

fn main() {
    //                heap-allocated String
    //                vvvvvvvvvvvvvvvvvvvvv
    let first_owner = String::from("hello");

    // let second_owner = first_owner;

    println!("{:?}, world!", first_owner);
}

// #[derive(Debug, Clone)]
// struct Foo;
