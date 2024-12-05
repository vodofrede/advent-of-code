use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input/2024-05.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    // parse
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().filter_map(|l| l.split_once('|')).fold(
        HashMap::<u64, HashSet<u64>>::new(),
        |mut rules, (before, after)| {
            rules
                .entry(after.parse().unwrap())
                .or_default()
                .insert(before.parse().unwrap());
            rules
        },
    );
    let updates = updates.lines().map(|l| {
        l.split(',')
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });
    let valid = updates.filter(|pages| pages.is_sorted_by(|a, b| rules[b].contains(a)));
    let sum = valid.map(|pages| pages[(pages.len() - 1) / 2]).sum::<u64>();

    println!("part 1: {sum}");
}

fn part2(input: &str) {
    // parse
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().filter_map(|l| l.split_once('|')).fold(
        HashMap::<u64, HashSet<u64>>::new(),
        |mut rules, (before, after)| {
            rules
                .entry(after.parse().unwrap())
                .or_default()
                .insert(before.parse().unwrap());
            rules
        },
    );
    let updates = updates.lines().map(|l| {
        l.split(',')
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });
    let invalid = updates.filter(|pages| !pages.is_sorted_by(|a, b| rules[b].contains(a)));
    let sorted = invalid.map(|mut pages| {
        pages.sort_by(|a, b| rules[b].contains(a).cmp(&true));
        pages
    });
    let sum = sorted
        .map(|pages| pages[(pages.len() - 1) / 2])
        .sum::<u64>();

    println!("part 1: {sum}");
}
