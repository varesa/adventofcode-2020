use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect(format!("Failed to open {}", filename).as_ref());

    let mut wanted: HashSet<i32> = HashSet::new();

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        let num = line.parse::<i32>().unwrap();

        if wanted.contains(&num) {
            println!("{}", num * (2020 - num));
        }

        let pair = 2020 - num;
        if pair >= 0 {
            wanted.insert(pair);
        }
    }
}
