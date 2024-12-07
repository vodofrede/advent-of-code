use std::fs;

fn main() {
    let input = fs::read_to_string("input/2024-07.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let equations = input
        .lines()
        .filter_map(|l| l.split_once(": "))
        .map(|(left, right)| {
            let left = left.parse::<u64>().unwrap();
            let right = right
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (left, right)
        });
    let valid = equations.filter(|(left, right)| {
        let perms = 1usize << (right.len() - 1);
        (0..perms).any(|mask| {
            right[1..]
                .iter()
                .enumerate()
                .fold(right[0], |left, (k, right)| match ((mask >> k) & 1) == 1 {
                    true => left + right,
                    false => left * right,
                })
                == *left
        })
    });
    let sum = valid.map(|(s, _)| s).sum::<u64>();

    println!("part 1: {sum}");
}
fn part2(input: &str) {
    let equations = input
        .lines()
        .filter_map(|l| l.split_once(": "))
        .map(|(left, right)| {
            let left = left.parse::<u64>().unwrap();
            let right = right
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (left, right)
        });
    let valid = equations.filter(|(left, right)| {
        let perms = 3usize.pow((right.len() - 1) as u32); // ternary mask
        (0..perms).any(|mask| {
            right[1..]
                .iter()
                .enumerate()
                .fold(right[0], |left, (k, right)| {
                    match (mask / (3usize.pow(k as u32))) % 3 {
                        0 => left + right,
                        1 => left * right,
                        _ => left * 10u64.pow(right.ilog(10) + 1) + right, // concat
                    }
                })
                == *left
        })
    });
    let sum = valid.map(|(s, _)| s).sum::<u64>();

    println!("part 1: {sum}");
}
