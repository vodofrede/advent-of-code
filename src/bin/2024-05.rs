use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/2024-05.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    // parse
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().filter_map(|l| l.split_once('|')).fold(
        HashMap::new(),
        |mut rules, (before, after)| {
            rules
                .entry(before)
                .and_modify(|v: &mut Vec<_>| v.push(after))
                .or_default();
            rules
        },
    );

    let valid = updates
        .lines()
        .map(|l| l.split(',').collect::<Vec<_>>())
        .filter(|pages| {
            for i in 0..pages.len() {
                let page = pages[i];
                let left = &pages[..i];
                if let Some(rules) = rules.get(page) {
                    if rules.iter().any(|after| left.contains(after)) {
                        return false;
                    }
                }
            }
            true
        });

    let sum = valid
        .map(|pages| pages[(pages.len() - 1) / 2].parse::<u64>().unwrap())
        .sum::<u64>();

    println!("part 1: {sum}");
}
fn part2(input: &str) {}
