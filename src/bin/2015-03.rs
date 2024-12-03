use std::{collections::HashSet, fs, iter};

fn main() {
    let input = fs::read_to_string("input/2015-03.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn visit(
    mut h: HashSet<(i64, i64)>,
    (x, y): (i64, i64),
    c: char,
) -> (HashSet<(i64, i64)>, (i64, i64)) {
    let pos = match c {
        '>' => (x + 1, y),
        'v' => (x, y - 1),
        '<' => (x - 1, y),
        '^' => (x, y + 1),
        _ => panic!(),
    };
    h.insert(pos);
    (h, pos)
}
fn part1(input: &str) {
    let count = input
        .chars()
        .fold((HashSet::from([(0, 0)]), (0, 0)), |(h, p), c| {
            visit(h, p, c)
        })
        .0
        .len();
    println!("part 1: {count}");
}
fn part2(input: &str) {
    let (santa, robot) = iter::zip(input.chars().step_by(2), input.chars().skip(1).step_by(2))
        .collect::<(Vec<_>, Vec<_>)>();
    let santa_visits = santa
        .iter()
        .fold((HashSet::from([(0, 0)]), (0, 0)), |(h, pos), c| {
            visit(h, pos, *c)
        })
        .0;
    let robot_visits = robot
        .iter()
        .fold((HashSet::from([(0, 0)]), (0, 0)), |(h, pos), c| {
            visit(h, pos, *c)
        })
        .0;
    let count = santa_visits.union(&robot_visits).count();
    println!("part 2: {count}");
}
