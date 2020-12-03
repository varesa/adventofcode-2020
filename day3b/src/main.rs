use std::env;
use std::fs;

fn check_slope(map: &String, dx: usize, dy: usize) -> usize {
    let mut pos_x = 0;
    let mut pos_y = 0;
    map
        .lines()
        .filter_map(|line| {
            let width = line.chars().count();
            let next = line.chars().nth(pos_x % width);

            if (pos_y) % dy != 0 {
                pos_y = pos_y + 1;
                return None;
            }
            
            pos_y = pos_y + 1;
            pos_x = pos_x + dx;

            match next.unwrap() {
                '#' => Some(()),
                _   => None,
            }
        })
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect(format!("Failed to open {}", filename).as_ref());

    let trees11 = check_slope(&input, 1, 1);
    let trees31 = check_slope(&input, 3, 1);
    let trees51 = check_slope(&input, 5, 1);
    let trees71 = check_slope(&input, 7, 1);
    let trees12 = check_slope(&input, 1, 2);
    println!("{} * {} * {} * {} * {} = {}", trees11, trees31, trees51, trees71, trees12, trees11*trees31*trees51*trees71*trees12);
}
