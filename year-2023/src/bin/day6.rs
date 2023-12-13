use eyes::parse;
use std::iter;

fn main() {
    let input = include_str!("../../input/day6.txt");
    println!("--- Results ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let [times, distances] = &input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>()[..] else { unreachable!() };
    iter::zip(times, distances)
        .map(|(time, distance)| {
            (0..=*time)
                .filter(|hold| hold * (time - hold) > *distance)
                .count() as i64
        })
        .product()
}

fn part2(input: &str) -> i64 {
    let [time, distance] = &input
        .lines()
        .map(|line| {
            let (_, n) = line.split_once(':').unwrap();
            n.replace(" ", "").parse::<i64>().unwrap()
        })
        .collect::<Vec<_>>()[..] else { unreachable!() };
    (0..=*time)
        .filter(|hold| hold * (time - hold) > *distance)
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLUTION_PART1: i64 = 288;
    const SOLUTION_PART2: i64 = 71503;
    const EXAMPLE_TEXT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn examples_part1() {
        assert_eq!(part1(EXAMPLE_TEXT), SOLUTION_PART1);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(part2(EXAMPLE_TEXT), SOLUTION_PART2);
    }
}
