use std::fmt::Write;

fn main() {
    let input = "yzbqklnj";
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let len = input.len();
    let mut input = input.to_string();
    let number = (1..)
        .find(move |i| {
            input.truncate(len);
            let _ = write!(input, "{}", i);
            let digest = md5::compute(&input);
            digest[0] == 0 && digest[1] == 0 && (digest[2] >> 4) == 0
        })
        .unwrap();
    println!("part 1: {number}");
}
fn part2(input: &str) {
    let len = input.len();
    let mut input = input.to_string();
    let number = (1..)
        .find(move |i| {
            input.truncate(len);
            let _ = write!(input, "{}", i);
            let digest = md5::compute(&input);
            digest[0] == 0 && digest[1] == 0 && digest[2] == 0
        })
        .unwrap();
    println!("part 2: {number}");
}
