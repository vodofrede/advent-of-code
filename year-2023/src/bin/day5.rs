use std::ops::Range;

use eyes::parse;

fn main() {
    let input = include_str!("../../input/day5.txt");
    println!("--- Results ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Map {
    ranges: Vec<Range<usize>>,
}

fn part1(input: &str) -> i64 {
    let mut parts = input.split("\n\n");
    let seeds = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    parts
        .map(|p| {
            p.lines()
                .skip(1)
                .map(|l| {
                    let (dst, src, len) = parse!(l, "{} {} {}", i64, i64, i64);
                    (src..src + len, dst)
                })
                .collect::<Vec<_>>()
        })
        .fold(seeds, |mut seeds, map| {
            for seed in seeds.iter_mut() {
                *seed = map
                    .iter()
                    .find_map(|(src, dst)| src.contains(seed).then(|| *seed + (dst - src.start)))
                    .unwrap_or(*seed);
            }
            seeds
        })
        .into_iter()
        .min()
        .unwrap() as i64
}

fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            part1(
                r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(part2(r""), 0);
    }
}
