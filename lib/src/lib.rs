use request::Request;
use std::{env, fs};

pub fn input(year: usize, day: usize) -> String {
    // check cache
    if let Ok(input) = fs::read_to_string(dbg!(format!("input/{year}-{day:02}.txt"))) {
        return input;
    }

    // check cookie exists
    let Ok(token) = env::var("AOC_TOKEN") else {
        panic!("Set env AOC_TOKEN to a valid token before proceeding.")
    };

    // fetch and store input
    let input = Request::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("Cookie", &format!("session={token}"))
        .send()
        .unwrap()
        .body;
    fs::write(format!("input/{year}-{day:2}.txt"), &input).unwrap();

    input
}
