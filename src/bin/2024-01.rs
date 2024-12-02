use regex::Regex;
use std::{collections::HashMap, fs, iter};

fn main() {
    let input = fs::read_to_string("input/2401.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    input
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| {
            (
                c[1].parse::<usize>().unwrap(),
                c[2].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn part1(input: &str) {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    let sum = iter::zip(left, right)
        .map(|(l, r)| l.max(r) - l.min(r))
        .sum::<usize>();
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
        .sum::<usize>();
    println!("part 2: {sum}");
}
