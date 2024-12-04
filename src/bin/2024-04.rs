use std::fs;

fn main() {
    let input = fs::read_to_string("input/2024-04.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    const DIRS: &[(isize, isize)] = &[
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    fn is_xmas(
        s: &[Vec<char>],
        (mut x, mut y): (isize, isize),
        (dx, dy): (isize, isize),
        (w, h): (isize, isize),
    ) -> bool {
        let mut r = 0;
        while r < 4 && 0 <= x && x < w && 0 <= y && y < h {
            if s[x as usize][y as usize] != "XMAS".chars().nth(r).unwrap() {
                return false;
            }
            (x, y) = (x + dx, y + dy);
            r += 1;
        }
        r == 4
    }

    let (w, h) = (input.find("\n").unwrap(), input.lines().count());
    let s = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for x in 0..w {
        for y in 0..h {
            for dir in DIRS {
                sum += is_xmas(&s, (x as isize, y as isize), *dir, (w as isize, h as isize)) as u64;
            }
        }
    }
    println!("part 1: {sum}");
}

fn part2(input: &str) {
    fn is_mas(
        s: &[Vec<char>],
        (x, y): (isize, isize),
        (dx, dy): (isize, isize),
        (w, h): (isize, isize),
    ) -> bool {
        let valid = |(x, y), (w, h)| 0 <= x && x < w && 0 <= y && y < h;
        let (xn, yn) = (x + dx, y + dy);
        let (xp, yp) = (x - dx, y - dy);

        valid((xn, yn), (w, h))
            && valid((xp, yp), (w, h))
            && s[x as usize][y as usize] == 'A'
            && (s[xp as usize][yp as usize] == 'M' && s[xn as usize][yn as usize] == 'S'
                || s[xp as usize][yp as usize] == 'S' && s[xn as usize][yn as usize] == 'M')
    }

    let (w, h) = (
        input.find("\n").unwrap() as isize,
        input.lines().count() as isize,
    );
    let s = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for x in 0..w {
        for y in 0..h {
            let x_mas = is_mas(&s, (x, y), (1, 1), (w, h)) && is_mas(&s, (x, y), (1, -1), (w, h));
            sum += x_mas as u64
        }
    }
    println!("part 2: {sum}");
}
