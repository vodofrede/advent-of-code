use core::str;
use std::{fs, iter};

fn main() {
    let input = fs::read_to_string("input/2015-05.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let count = input
        .lines()
        .filter(|l| {
            let vowels = l.chars().filter(|c| "aeiou".contains(*c)).count();
            let double = iter::zip(l.chars(), l[1..].chars()).any(|(a, b)| a == b);
            let bad = l
                .as_bytes()
                .windows(2)
                .any(|b| b == b"ab" || b == b"cd" || b == b"pq" || b == b"xy");

            vowels >= 3 && double && !bad
        })
        .count();
    println!("part 1: {count}");
}
fn part2(input: &str) {
    let count = input
        .lines()
        .filter(|l| {
            let has_pairs = l
                .as_bytes()
                .windows(2)
                .enumerate()
                .any(|(i, p)| l[i + 2..].contains(str::from_utf8(p).unwrap()));
            let sandwich = l.as_bytes().windows(3).any(|b| b[0] == b[2]);

            has_pairs && sandwich
        })
        .count();
    println!("part 2: {count}");
}
