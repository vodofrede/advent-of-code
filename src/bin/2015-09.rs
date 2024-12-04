use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/2015-09.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    // construct edges
    let re = Regex::new(r"(.+) to (.+) = (\d+)").unwrap();
    let edges = input
        .lines()
        .filter_map(|l| re.captures(l))
        .map(|c| {
            (
                c[1].to_string(),
                c[2].to_string(),
                c[3].parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let cities = edges
        .iter()
        .flat_map(|e| [e.0.clone(), e.1.clone()])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let edges = edges
        .into_iter()
        .map(|e| {
            (
                cities.iter().position(|c| *c == e.0).unwrap(),
                cities.iter().position(|c| *c == e.1).unwrap(),
                e.2,
            )
        })
        .collect::<Vec<_>>();

    // floyd-warshall
    let mut dist = vec![vec![u64::MAX; cities.len()]; cities.len()];
    for edge in &edges {
        dist[edge.0][edge.1] = edge.2;
        dist[edge.1][edge.0] = edge.2;
    }
    for i in 0..cities.len() {
        dist[i][i] = 0;
    }
    for k in 0..cities.len() {
        for i in 0..cities.len() {
            for j in 0..cities.len() {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j]
                }
            }
        }
    }

    // all permutations
    let shortest = (0..cities.len())
        .permutations(cities.len())
        .map(|perm| {
            perm.windows(2)
                .map(|path| dist[path[0]][path[1]])
                .sum::<u64>()
        })
        .min()
        .unwrap();

    println!("part 1: {shortest}");
}
fn part2(input: &str) {}
