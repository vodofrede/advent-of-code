use regex::Regex;
use std::{fs, iter};

fn main() {
    let input = fs::read_to_string("input/2024-03.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = re
        .captures_iter(input)
        .map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
        .sum::<i64>();
    println!("part 1: {sum}");
}
fn part2(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let sum = re
        .captures_iter(input)
        .fold((0, true), |(sum, enable), c| match (&c[0], enable) {
            ("do()", _) => (sum, true),
            ("don't()", _) => (sum, false),
            (_, true) => (
                sum + c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap(),
                enable,
            ),
            _ => (sum, enable),
        })
        .0;
    println!("part 2: {sum}");
}
