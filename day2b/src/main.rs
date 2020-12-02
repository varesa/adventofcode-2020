
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

        let pos1 = captures[1].parse::<usize>().unwrap();
        let pos2 = captures[2].parse::<usize>().unwrap();
        let character = &captures[3].chars().nth(0).unwrap();
        let password = &captures[4];

        let first = &password.chars().nth(pos1 - 1).unwrap() == character;
        let second = &password.chars().nth(pos2 - 1).unwrap() == character;

        if (first || second) && !(first && second)  {
            return Some(password.into());
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
