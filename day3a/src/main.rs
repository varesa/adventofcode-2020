use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect(format!("Failed to open {}", filename).as_ref());

    let mut pos_x = 0;

    let trees = input
        .lines()
        .filter_map(|line| {
            let width = line.chars().count();
            let next = line.chars().nth(pos_x % width);
            println!("{}", next.unwrap());
            pos_x = pos_x + 3;
            match next.unwrap() {
                '#' => Some(()),
                _   => None,

            }
        })
        .count();

    println!("{}", trees);

}
