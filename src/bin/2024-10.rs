use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input/2024-10.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, r)| {
            r.char_indices()
                .map(move |(x, c)| ((x as isize, y as isize), u32::from(c) - 48))
        })
        .collect::<HashMap<_, _>>();

    fn dfs((x, y): (isize, isize), grid: &HashMap<(isize, isize), u32>) -> u64 {
        let mut queue = VecDeque::from([(x, y)]);
        let mut visited = HashSet::new();
        let mut count = 0;

        while let Some((x, y)) = queue.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }
            let Some(p) = grid.get(&(x, y)) else { continue };
            if *p == 9 {
                count += 1;
                continue;
            }
            let neighbors = [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)];
            for (nx, ny) in neighbors {
                let Some(n) = grid.get(&(nx, ny)) else {
                    continue;
                };
                if n - p == 1 {
                    queue.push_back((nx, ny));
                }
            }
        }

        count
    }

    let valid = grid
        .iter()
        .filter_map(|(p, h)| (*h == 0).then_some(*p))
        .map(|p| dfs(p, &grid))
        .sum::<u64>();

    println!("part 1: {valid}");
}
fn part2(input: &str) {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, r)| {
            r.char_indices()
                .map(move |(x, c)| ((x as isize, y as isize), u32::from(c) - 48))
        })
        .collect::<HashMap<_, _>>();

    fn dfs((x, y): (isize, isize), grid: &HashMap<(isize, isize), u32>) -> u64 {
        let mut queue = VecDeque::from([(x, y)]);
        let mut count = 0;

        while let Some((x, y)) = queue.pop_front() {
            let Some(p) = grid.get(&(x, y)) else { continue };
            if *p == 9 {
                count += 1;
                continue;
            }
            let neighbors = [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)];
            for (nx, ny) in neighbors {
                let Some(n) = grid.get(&(nx, ny)) else {
                    continue;
                };
                if n - p == 1 {
                    queue.push_back((nx, ny));
                }
            }
        }

        count
    }

    let valid = grid
        .iter()
        .filter_map(|(p, h)| (*h == 0).then_some(*p))
        .map(|p| dfs(p, &grid))
        .sum::<u64>();

    println!("part 2: {valid}");
}
