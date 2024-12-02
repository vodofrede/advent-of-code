use std::{fs, iter};

fn main() {
    let input = fs::read_to_string("input/2024-02.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let safe = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|vs| problem(vs).is_none())
        .count();

    println!("part 1: {safe}");
}
fn part2(input: &str) {
    let safe = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|vs| match problem(vs) {
            None => true,
            Some(_pos) => (0..vs.len()).any(|i| {
                let mut vs = vs.clone();
                vs.remove(i);
                problem(&vs).is_none()
            }),
        })
        .count();

    println!("part 2: {safe}");
}

fn problem(vs: &[i64]) -> Option<usize> {
    let order = vs[0].cmp(&vs[1]);
    iter::zip(vs, &vs[1..]).position(|(a, b)| {
        let monotonic = a.cmp(b) == order;
        let close = a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1;
        !monotonic || !close
    })
}
