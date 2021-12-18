use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let parts: Vec<&str> = data.split("\n\n").collect();

    let mut rules: HashMap<[char; 2], [[char; 2]; 2]> = HashMap::new();

    for line in parts[1].split("\n") {
        let line_parts: Vec<&str> = line.split(" -> ").collect();
        let before: Vec<char> = line_parts[0].chars().collect();
        let after: Vec<char> = line_parts[1].chars().collect();

        rules.insert(
            [before[0], before[1]],
            [[before[0], after[0]], [after[0], before[1]]],
        );
    }

    let polymer: Vec<char> = parts[0].chars().collect();
    let first_letter: char = polymer[0];
    let windows: Vec<[char; 2]> = polymer[..]
        .windows(2)
        .flat_map(<[char; 2]>::try_from)
        .collect();
    let mut count: HashMap<[char; 2], u64> = HashMap::new();

    for pattern in windows {
        *count.entry(pattern).or_default() += 1;
    }

    for _ in 0..40 {
        let mut new_count: HashMap<[char; 2], u64> = HashMap::new();

        for (pattern, pattern_count) in count.clone() {
            let rule: [[char; 2]; 2] = *rules.get(&pattern).unwrap();
            *new_count.entry(rule[0]).or_default() += pattern_count;
            *new_count.entry(rule[1]).or_default() += pattern_count;
        }

        count = new_count;
    }

    let mut freq: HashMap<char, u64> = HashMap::new();
    for (pattern, pattern_count) in count {
        *freq.entry(pattern[1]).or_default() += pattern_count;
    }
    *freq.entry(first_letter).or_default() += 1;

    let res: u64 = freq.clone().values().max().unwrap() - freq.values().min().unwrap();

    println!("{:?}", res);
}
