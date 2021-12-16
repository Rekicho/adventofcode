use std::fs;
use std::collections::HashMap;
use std::convert::TryFrom;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let parts: Vec<&str> = data.split("\n\n").collect();
    let mut polymer: Vec<char> = parts[0].chars().collect();
    let mut rules: HashMap<[char; 2], [char; 2]> = HashMap::new();

    for line in parts[1].split("\n") {
        let line_parts: Vec<&str> = line.split(" -> ").collect();
        let before: Vec<char> = line_parts[0].chars().collect();
        let after: Vec<char> = line_parts[1].chars().collect();

        rules.insert([before[0], before[1]], [after[0], before[1]]);
    }

    for _ in 0..10 {
        let windows: Vec<[char; 2]> = polymer[..].windows(2).flat_map(<[char; 2]>::try_from).collect();
        let new_polymer = windows.into_iter().fold(Vec::new(), |mut new_vec: Vec<char>, window| { new_vec.extend(rules.get(&window).unwrap());
                                                                                                  new_vec });
        polymer = Vec::from([polymer[0]]);
        polymer.extend(new_polymer);
    }

    let mut freq: HashMap<char, u32> = HashMap::new();
    for letter in polymer {
        *freq.entry(letter).or_default() += 1;
    }
    let res: u32 = freq.clone().values().max().unwrap() - freq.values().min().unwrap();

    println!("{:?}", res);
}