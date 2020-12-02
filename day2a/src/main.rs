
#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;

use regex::Regex;

fn get_password(s: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*)-(.*) (.): (.*)$").unwrap();
    }

    if let Some(captures) = RE.captures(s) {
        //dbg!(&captures);

        let min = captures[1].parse::<usize>().unwrap();
        let max = captures[2].parse::<usize>().unwrap();
        let character = &captures[3];
        let password = &captures[4];

        let count = password.matches(character).count();
        if min <= count && count <= max {
            return Some(password.into())
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect(format!("Failed to open {}", filename).as_ref());

    let passwords = input
        .lines()
        .filter_map(|line| get_password(line))
        .count();

    println!("{}", passwords);
}
