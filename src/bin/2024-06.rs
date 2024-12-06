use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/2024-06.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    dims: (usize, usize),
}
impl Map {
    fn contains(&self, (x, y): (isize, isize)) -> bool {
        0 <= x && x < self.dims.0 as isize && 0 <= y && y < self.dims.1 as isize
    }
    fn get(&self, (x, y): (isize, isize)) -> Option<char> {
        self.contains((x, y))
            .then(|| self.grid.get(y as usize)?.get(x as usize).copied())?
    }
    fn get_mut(&mut self, (x, y): (isize, isize)) -> Option<&mut char> {
        self.contains((x, y))
            .then(|| self.grid.get_mut(y as usize)?.get_mut(x as usize))?
    }
}
fn part1(input: &str) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dims = (grid[0].len(), grid.len());
    let mut guard = (0..grid.len())
        .find_map(|y| {
            (0..grid[y].len()).find_map(|x| (grid[y][x] == '^').then_some((x as isize, y as isize)))
        })
        .unwrap();
    let mut direction = (0, -1);
    let map = Map { grid, dims };
    let visited = (0..)
        .map_while(|_| loop {
            let (x, y) = guard;
            let (dx, dy) = direction;
            let (nx, ny) = (x + dx, y + dy);
            let next = map.get((nx, ny))?;
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
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dims = (grid[0].len(), grid.len());
    let guard = (0..grid.len())
        .find_map(|y| {
            (0..grid[y].len()).find_map(|x| (grid[y][x] == '^').then_some((x as isize, y as isize)))
        })
        .unwrap();
    let map = Map { grid, dims };

    fn walk(map: &Map, start: (isize, isize), simulation: bool) -> Option<HashSet<(isize, isize)>> {
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
            match map.get((nx, ny)) {
                Some('.') | Some('^') => {
                    if !simulation && !obstacles.contains(&(nx, ny)) {
                        let mut map = map.clone();
                        *map.get_mut((nx, ny))? = '#';
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
