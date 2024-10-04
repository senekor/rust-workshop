// #![feature(ptr_metadata)]

trait Animal {
    fn make_sound(&self);
}
struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("woof!")
    }
}
struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("meow!")
    }
}

fn get_animals() -> Vec<Animal> {
    vec![Dog, Dog, Cat]
}

fn main() {
    for animal in get_animals() {
        animal.make_sound();
    }
}

// two problems:
// - Cat and Dog may have different size
// - information about implementation needed at runtime

// let p = animal as *const dyn Animal;
// let parts = p.to_raw_parts();
// print!("{parts:?} ");
