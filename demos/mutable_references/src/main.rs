// source:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    todo!("modify {some_string}")
    // some_string.push_str(", world");
}
