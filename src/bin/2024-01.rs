use std::{collections::HashMap, fs, iter};

fn main() {
    let input = fs::read_to_string("input/2024-01.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .filter_map(|l| l.split_once("   "))
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect()
}

fn part1(input: &str) {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    let sum = iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u64>();
    println!("part 1: {sum}");
}
fn part2(input: &str) {
    let (left, right) = parse(input);

    let counts = right.into_iter().fold(HashMap::new(), |mut map, v| {
        map.entry(v).and_modify(|freq| *freq += 1).or_insert(1);
        map
    });

    let sum = left
        .iter()
        .map(|v| v * counts.get(v).unwrap_or(&0))
        .sum::<u64>();
    println!("part 2: {sum}");
}
