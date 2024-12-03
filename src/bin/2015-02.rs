use regex::Regex;
use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("input/2015-02.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let sum = input
        .lines()
        .filter_map(|l| re.captures(l))
        .map(|c| c.extract().1.map(|v| v.parse::<u64>().unwrap()))
        .map(|[l, w, h]| {
            let area = 2 * l * w + 2 * w * h + 2 * h * l;
            let slack = [l * w, w * h, h * l].into_iter().min().unwrap();
            area + slack
        })
        .sum::<u64>();
    println!("part 1: {sum}");
}
fn part2(input: &str) {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let sum = input
        .lines()
        .filter_map(|l| re.captures(l))
        .map(|c| c.extract::<3>().1.map(|v| v.parse::<u64>().unwrap()))
        .map(|mut d| {
            d.sort();
            let wrap = 2 * (d[0] + d[1]);
            let bow = d[0] * d[1] * d[2];
            wrap + bow
        })
        .sum::<u64>();
    println!("part 1: {sum}");
}
