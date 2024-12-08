use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input/2024-08.txt").unwrap();
    part1(&input);
    part2(&input);
}

type Pos = (isize, isize);
fn in_bounds((x, y): Pos, (w, h): (isize, isize)) -> bool {
    0 <= x && x < w && 0 <= y && y < h
}
fn part1(input: &str) {
    let (w, h) = (
        input.find('\n').unwrap() as isize,
        input.lines().count() as isize,
    );
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect::<HashMap<_, _>>();
    let types = input
        .chars()
        .unique()
        .filter(|c| c != &'.')
        .map(|t| map.iter().filter_map(move |(k, v)| (*v == t).then_some(*k)));

    let mut unique = HashSet::new();
    for antennas in types {
        for p in antennas.permutations(2) {
            let ((x1, y1), (x2, y2)) = (p[0], p[1]);
            let (dx, dy) = (x2 - x1, y2 - y1);
            let (na, nb) = ((x1 - dx, y1 - dy), (x2 + dx, y2 + dy));
            unique.extend([na, nb].into_iter().filter(|n| in_bounds(*n, (w, h))))
        }
    }
    let count = unique.len();

    println!("part 1: {count}");
}
fn part2(input: &str) {
    let (w, h) = (
        input.find('\n').unwrap() as isize,
        input.lines().count() as isize,
    );
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect::<HashMap<_, _>>();
    let types = input
        .chars()
        .unique()
        .filter(|c| c != &'.')
        .map(|t| map.iter().filter_map(move |(k, v)| (*v == t).then_some(*k)));

    let mut unique = HashSet::new();
    for i in 0..w {
        for antennas in types.clone() {
            for p in antennas.permutations(2) {
                let ((x1, y1), (x2, y2)) = (p[0], p[1]);
                let (dx, dy) = (i * (x2 - x1), i * (y2 - y1));
                let (na, nb) = ((x1 - dx, y1 - dy), (x2 + dx, y2 + dy));
                unique.extend([na, nb].into_iter().filter(|n| in_bounds(*n, (w, h))))
            }
        }
    }
    let count = unique.len();

    println!("part 2: {count}");
}
