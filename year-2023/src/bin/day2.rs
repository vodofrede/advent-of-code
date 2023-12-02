use eyes::parse;

fn main() {
    let input = include_str!("../../input/day2.txt");
    println!("--- Results ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let (game, sets) = line.split_once(':').unwrap();
            let id = parse!(game, "Game {}", i64);
            sets.split(';')
                .all(|set| {
                    let (red, green, blue) = set.split(',').fold((0, 0, 0), |(r, g, b), hand| {
                        let (amount, color) = parse!(hand.trim(), "{} {}", i64, String);
                        match color.as_str() {
                            "red" => (r + amount, g, b),
                            "green" => (r, g + amount, b),
                            "blue" => (r, g, b + amount),
                            _ => panic!("unknown color, check input"),
                        }
                    });
                    red <= 12 && green <= 13 && blue <= 14
                })
                .then_some(id)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (_game, sets) = line.split_once(':').unwrap();
            let (red, green, blue) = sets
                .split(';')
                .map(|set| {
                    let (red, green, blue) = set.split(',').fold((0, 0, 0), |(r, g, b), hand| {
                        let (amount, color) = parse!(hand.trim(), "{} {}", i64, String);
                        match color.as_str() {
                            "red" => (r + amount, g, b),
                            "green" => (r, g + amount, b),
                            "blue" => (r, g, b + amount),
                            _ => panic!("unknown color, check input"),
                        }
                    });
                    (red, green, blue)
                })
                .fold((0, 0, 0), |(mr, mg, mb), (r, g, b)| {
                    (mr.max(r), mg.max(g), mb.max(b))
                });
            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            part1(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            part2(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
