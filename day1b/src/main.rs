use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect(format!("Failed to open {}", filename).as_ref());
    let mut numbers: HashSet<i32> = HashSet::new();

    input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .for_each(|number| { numbers.insert(number); });
    
    'outer: for first in &numbers {
        for second in &numbers {
            let third = 2020 - first - second;
            if numbers.contains(&third) {
                println!("{}", first * second * third);
                break 'outer;
            }
        }
    }
}
