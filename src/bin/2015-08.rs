use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/2015-08.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let re = Regex::new(r#"\\\\|\\"|\\x.."#).unwrap();
    let diffs = input
        .lines()
        .map(|l| 2 + re.find_iter(l).map(|m| m.len() - 1).sum::<usize>())
        .sum::<usize>();
    println!("part 1: {diffs}");
}
fn part2(input: &str) {
    let re = Regex::new(r#""|\\"#).unwrap();
    let diffs = input
        .lines()
        .map(|l| 2 + re.find_iter(l).count())
        .sum::<usize>();
    println!("part 1: {diffs}");
}
