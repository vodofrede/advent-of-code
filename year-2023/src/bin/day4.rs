use std::{collections::HashSet, iter};

fn main() {
    let input = include_str!("../../input/day4.txt");
    println!("--- Results ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(':').unwrap();
            let (winners, numbers) = card.split_once('|').unwrap();
            (
                winners.trim().split_whitespace().collect::<HashSet<_>>(),
                numbers.trim().split_whitespace().collect::<HashSet<_>>(),
            )
        })
        .filter_map(|(winners, numbers)| {
            let w = winners.intersection(&numbers).count();
            (w > 0).then(|| 2i64.pow(w as u32 - 1))
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(':').unwrap();
            let (winners, numbers) = card.split_once('|').unwrap();
            (
                winners.trim().split_whitespace().collect::<HashSet<_>>(),
                numbers.trim().split_whitespace().collect::<HashSet<_>>(),
            )
        })
        .fold((vec![0], 0), |(mut queue, cards), (winners, numbers)| {
            let w = winners.intersection(&numbers).count() as i64;
            let extra = queue.pop().unwrap();

            (queue, 1 + cards + extra)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            part1(
                r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            part2(
                r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }
}
