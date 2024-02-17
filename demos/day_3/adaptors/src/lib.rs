// solution for
// https://adventofcode.com/2022/day/1

pub fn get_solution(input: &str) -> u32 {
    let mut max_calories = 0;

    for elf in input.split("\n\n") {
        let mut calories_sum = 0;

        for line in elf.lines() {
            let calories = line.parse::<u32>().unwrap();
            calories_sum += calories;
        }
        if calories_sum > max_calories {
            max_calories = calories_sum;
        }
    }

    max_calories
}

#[test]
fn solution_is_correct() {
    static SAMPLE_INPUT: &str = include_str!("sample.txt");

    assert_eq!(get_solution(SAMPLE_INPUT), 24000);
}
