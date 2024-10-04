#![allow(unused)]

fn main() {
    let bad_separator = "12,8";
    let bad_length = "-12x8";
    let bad_width = "12x-8";
    let good = "12x8";

    let input = good;

    println!("area: {}", calculate_area(input));
}

fn calculate_area(input: &str) -> usize {
    let (length, width) = input.split_once('x').unwrap();

    let length = parse_number(length);
    let width = parse_number(width);

    length * width
}

fn parse_number(num: &str) -> usize {
    num.parse().unwrap()
}
