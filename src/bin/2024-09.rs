use itertools::Itertools;
use std::{fs, iter};

fn main() {
    let input = fs::read_to_string("input/2024-09.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let disk = input.trim().chars().map(|c| (u32::from(c) - 48) as usize);
    let (files, free) = (disk.clone().step_by(2), disk.skip(1).step_by(2));
    let flat_files = iter::zip(0.., files).map(|(i, f)| iter::repeat(i).take(f));
    let flat_free = free.map(|f| iter::repeat(-1).take(f));
    let mut data: Vec<_> = flat_files.interleave(flat_free).flatten().collect();

    let (mut i, mut j) = (0, data.len() - 1);
    while i < j {
        if data[i] == -1 {
            while data[j] == -1 {
                j -= 1;
            }
            data[i] = data[j];
            data[j] = -1;
        }
        i += 1;
    }

    let checksum = data
        .into_iter()
        .take_while(|n| *n != -1)
        .enumerate()
        .map(|(i, n)| i as u64 * n as u64)
        .sum::<u64>();
    println!("part 1: {checksum}");
}
fn part2(input: &str) {
    let disk = input.trim().chars().map(|c| (u32::from(c) - 48) as usize);
    let mut data: Vec<_> = iter::zip(1.., disk)
        .map(|(i, n)| (if i % 2 == 1 { i / 2 } else { -1i32 }, n))
        .collect();

    let mut i = data.len() - 1;
    while i > 0 {
        let (id, size) = data[i];
        if id == -1 {
            i -= 1;
            continue;
        }
        if let Some(j) = data[0..i].iter().position(|&(id, s)| id == -1 && size <= s) {
            let s = data[j].1;
            data[j] = (id, size);
            data[i] = (-1, size);
            if size < s {
                data.insert(j + 1, (-1, s - size));
            }
        }

        i -= 1
    }
    println!("{data:?}");

    let checksum = data
        .iter()
        .flat_map(|&(id, s)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
        .sum::<usize>();
    println!("part 1: {checksum}");
}
