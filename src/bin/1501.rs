use std::fs;

fn main() {
    let input = fs::read_to_string("input/1501.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let sum = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        })
        .sum::<i64>();
    println!("part 1: {sum}");
}
fn part2(input: &str) {
    let pos = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        })
        .scan(0, |floor, x| {
            *floor += x;
            Some(*floor)
        })
        .position(|floor| floor < 0)
        .unwrap()
        + 1; // problem is 1-indexed

    println!("part 1: {pos}");
}
