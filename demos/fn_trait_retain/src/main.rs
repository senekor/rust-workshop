//

//

fn main() {
    let x = 3;

    let mut nums: Vec<_> = (1..=16).collect();

    nums.retain(|elem| elem % x == 0);

    println!("remaining: {:?}", nums);
}
