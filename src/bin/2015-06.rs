use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/2015-06.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn mod_lights<T>(
    lights: &mut [T],
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    f: fn(&mut T),
) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            f(&mut lights[y * 1000 + x])
        }
    }
}
fn part1(input: &str) {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let lights = input
        .lines()
        .filter_map(|l| re.captures(l))
        .fold(vec![false; 1000 * 1000], |mut lights, c| {
            let x1 = c[2].parse::<usize>().unwrap();
            let y1 = c[3].parse::<usize>().unwrap();
            let x2 = c[4].parse::<usize>().unwrap();
            let y2 = c[5].parse::<usize>().unwrap();

            match &c[1] {
                "turn on" => mod_lights(&mut lights, (x1, y1), (x2, y2), |l| *l = true),
                "turn off" => mod_lights(&mut lights, (x1, y1), (x2, y2), |l| *l = false),
                "toggle" => mod_lights(&mut lights, (x1, y1), (x2, y2), |l| *l = !*l),
                _ => panic!(),
            }
            lights
        })
        .iter()
        .filter(|l| **l)
        .count();
    println!("part 1: {lights}");
}
fn part2(input: &str) {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let brightness = input
        .lines()
        .filter_map(|l| re.captures(l))
        .fold(vec![0u64; 1000 * 1000], |mut lights, c| {
            let x1 = c[2].parse::<usize>().unwrap();
            let y1 = c[3].parse::<usize>().unwrap();
            let x2 = c[4].parse::<usize>().unwrap();
            let y2 = c[5].parse::<usize>().unwrap();

            match &c[1] {
                "turn on" => mod_lights(&mut lights, (x1, y1), (x2, y2), |l| *l += 1),
                "turn off" => mod_lights(&mut lights, (x1, y1), (x2, y2), |l| {
                    *l = l.saturating_sub(1)
                }),
                "toggle" => mod_lights(&mut lights, (x1, y1), (x2, y2), |l| *l += 2),
                _ => panic!(),
            }
            lights
        })
        .iter()
        .sum::<u64>();
    println!("part 2: {brightness}");
}
