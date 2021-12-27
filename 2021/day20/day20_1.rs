use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let parts: Vec<&str> = data.split("\n\n").collect();

    let algorithm: Vec<bool> = parts[0].chars().map(|ch| ch == '#').collect();
    let mut image: HashMap<(isize, isize), bool> = HashMap::new();

    for (i, line) in parts[1].split("\n").enumerate() {
        for (j, ch) in line.chars().enumerate() {
            image.insert((i as isize, j as isize), ch == '#');
        }
    }

    let not_present_value: [bool; 2];

    match (algorithm[0], algorithm[algorithm.len() - 1]) {
        (false, _) => not_present_value = [false, false],
        (true, false) => not_present_value = [false, true],
        // vv this should never happen, or there will be infinite lit pixels
        (true, true) => not_present_value = [true, true],
    }

    for i in 0..2 {
        let parity = i % 2;

        let mut new_image: HashMap<(isize, isize), bool> = HashMap::new();
        let mut to_evaluate: HashSet<(isize, isize)> = HashSet::new();

        for ((x, y), _) in &image {
            for xx in x - 1..x + 2 {
                for yy in y - 1..y + 2 {
                    to_evaluate.insert((xx, yy));
                }
            }
        }

        for (x, y) in to_evaluate {
            let mut binary: String = String::new();

            for xx in x - 1..x + 2 {
                for yy in y - 1..y + 2 {
                    match image.get(&(xx, yy)) {
                        Some(true) => binary.push('1'),
                        Some(false) => binary.push('0'),
                        None => binary.push(if not_present_value[parity] { '1' } else { '0' }),
                    }
                }
            }

            let index: usize = usize::from_str_radix(&binary, 2).unwrap();

            new_image.insert((x, y), algorithm[index]);
        }

        image = new_image;
    }

    let res = image.values().filter(|pixel| **pixel).count();

    println!("{}", res);
}
