//

use std::array;

fn main() {
    println!("{:?}", 42.clone_10_times());
    println!();
    println!("{:?}", String::from("hello").clone_10_times());
    println!();
    println!("{:?}", vec![1, 2].clone_10_times());
}

trait Clone10Times: Sized {
    fn clone_10_times(self) -> [Self; 10];
}

impl Clone10Times for i32 {
    fn clone_10_times(self) -> [Self; 10] {
        array::from_fn(|_| self.clone())
    }
}
impl Clone10Times for String {
    fn clone_10_times(self) -> [Self; 10] {
        array::from_fn(|_| self.clone())
    }
}
impl Clone10Times for Vec<i32> {
    fn clone_10_times(self) -> [Self; 10] {
        array::from_fn(|_| self.clone())
    }
}
