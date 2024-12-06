use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input/2024-06.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect::<HashMap<_, _>>();
    let mut guard = *map.iter().find(|(_, c)| **c == '^').unwrap().0;
    let mut direction = (0, -1);
    let visited = (0..)
        .map_while(|_| loop {
            let (x, y) = guard;
            let (dx, dy) = direction;
            let (nx, ny) = (x + dx, y + dy);
            let next = map.get(&(nx, ny))?;
            match next {
                '.' | '^' => {
                    guard = (nx, ny);
                    return Some(guard);
                }
                '#' => {
                    direction = (-dy, dx);
                    continue;
                }
                _ => return None,
            }
        })
        .collect::<HashSet<_>>()
        .len();
    println!("part 1: {visited}");
}
fn part2(input: &str) {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect::<HashMap<_, _>>();
    let guard = *map.iter().find(|(_, c)| **c == '^').unwrap().0;

    fn walk(
        map: &HashMap<(isize, isize), char>,
        start: (isize, isize),
        simulation: bool,
    ) -> Option<HashSet<(isize, isize)>> {
        let mut obstacles = HashSet::new();
        let mut visited = HashSet::new();

        let (mut x, mut y) = start;
        let (mut dx, mut dy) = (0, -1);
        loop {
            if visited.contains(&((x, y), (dx, dy))) {
                return None;
            }
            visited.insert(((x, y), (dx, dy)));
            let (nx, ny) = (x + dx, y + dy);
            match map.get(&(nx, ny)) {
                Some('.') | Some('^') => {
                    if !simulation && !obstacles.contains(&(nx, ny)) && (x, y) != start {
                        let mut map = map.clone();
                        *map.get_mut(&(nx, ny))? = '#';
                        if walk(&map, start, true).is_none() {
                            obstacles.insert((nx, ny));
                        }
                    }
                    (x, y) = (nx, ny);
                }
                Some('#') => {
                    (dx, dy) = (-dy, dx);
                }
                _ => return Some(obstacles),
            }
        }
    }

    let obstacles = walk(&map, guard, false).unwrap().len();
    println!("part 2: {obstacles}");
}
