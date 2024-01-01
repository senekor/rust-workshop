#![allow(unused)]

// What does this program print?
//
// hint: The destructor, aka 'drop', of a value is run when its owner
// goes out of scope.

// declaring a zero-sized struct
struct Foo;

// writing a custom destructor for demo-purposes
// (Rust-lingo: "implementing the Drop trait")
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo's destructor was run!")
    }
}

fn main() {
    let x = Foo;
    {
        let y = x; // What happens if you comment this line?
    } // y goes out of scope

    println!("Hello, world!");
} // x goes out of scope
