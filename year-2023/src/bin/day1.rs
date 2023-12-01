fn main() {
    let input = include_str!("../../input/day1.txt");
    println!("--- Results ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim_matches(|c: char| c.is_alphabetic()))
        .map(|line| &line[..1].parse().unwrap() * 10 + &line[line.len() - 1..].parse().unwrap())
        .sum::<i32>() as i64
}

fn part2(input: &str) -> i64 {
    const SPELLINGS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            let mut numbers = vec![];

            for ci in 0..line.len() {
                let c = line[ci..].chars().next().unwrap();
                if c.is_numeric() {
                    numbers.push(c.to_digit(10).unwrap() as u32);
                    continue;
                }
                for (i, spelling) in SPELLINGS.iter().enumerate() {
                    if line[ci..].starts_with(spelling) {
                        numbers.push((i + 1) as u32)
                    }
                }
            }

            numbers
        })
        .map(|ns| ns[0] * 10 + ns.last().unwrap())
        .sum::<u32>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            part1(
                r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            part2(
                r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
