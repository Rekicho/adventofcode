#![feature(drain_filter)]
use std::collections::HashMap;
use std::fs;

fn calculate_output_code(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(" | ").collect();
    let mut patterns: Vec<&str> = parts[0].split(" ").collect();
    let output: Vec<&str> = parts[1].split(" ").collect();

    // Use "rules" to discover number representation
    let mut number_repr: HashMap<Vec<char>, u32> = HashMap::new();

    // 1 -> len == 2
    let mut one: Vec<char> = patterns
        .drain_filter(|x| x.chars().count() == 2)
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    one.sort();
    number_repr.insert(one.clone(), 1);

    // 4 -> len == 4
    let mut four: Vec<char> = patterns
        .drain_filter(|x| x.chars().count() == 4)
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    four.sort();
    number_repr.insert(four.clone(), 4);

    // 7 -> len == 3
    let mut seven: Vec<char> = patterns
        .drain_filter(|x| x.chars().count() == 3)
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    seven.sort();
    number_repr.insert(seven.clone(), 7);

    // 8 -> len == 7
    let mut eight: Vec<char> = patterns
        .drain_filter(|x| x.chars().count() == 7)
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    eight.sort();
    number_repr.insert(eight.clone(), 8);

    // 3 -> len == 5 && has all in 7
    let mut three: Vec<char> = patterns
        .drain_filter(|x| {
            x.chars().count() == 5
                && seven
                    .clone()
                    .into_iter()
                    .all(|ch| x.chars().collect::<Vec<char>>().contains(&ch))
        })
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    three.sort();
    number_repr.insert(three.clone(), 3);

    // 6 -> len == 6 && not has all in 7
    let mut six: Vec<char> = patterns
        .drain_filter(|x| {
            x.chars().count() == 6
                && !seven
                    .clone()
                    .into_iter()
                    .all(|ch| x.chars().collect::<Vec<char>>().contains(&ch))
        })
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    six.sort();
    number_repr.insert(six.clone(), 6);

    // 9 -> len == 6 && has all in 3
    let mut nine: Vec<char> = patterns
        .drain_filter(|x| {
            x.chars().count() == 6
                && three
                    .clone()
                    .into_iter()
                    .all(|ch| x.chars().collect::<Vec<char>>().contains(&ch))
        })
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    nine.sort();
    number_repr.insert(nine.clone(), 9);

    // 0 -> len == 6
    let mut zero: Vec<char> = patterns
        .drain_filter(|x| x.chars().count() == 6)
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    zero.sort();
    number_repr.insert(zero.clone(), 0);

    // 5 -> all in number exist in 6
    let mut five: Vec<char> = patterns
        .drain_filter(|x| x.chars().into_iter().all(|ch| six.contains(&ch)))
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    five.sort();
    number_repr.insert(five.clone(), 5);

    // 2 -> remaing number
    let mut two: Vec<char> = patterns[0].chars().collect();
    two.sort();
    number_repr.insert(two.clone(), 2);

    // let res = output.into_iter().fold(0, |acc, number| acc*10 + number_repr.get(&number.chars().collect::<Vec<char>>()).unwrap());
    output.into_iter().fold(0, |acc, number| {
        acc * 10
            + number_repr
                .get(&{
                    let mut chars = number.chars().collect::<Vec<char>>();
                    chars.sort();
                    chars
                })
                .unwrap()
    })
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let res = data
        .split("\n")
        .fold(0, |acc, line| acc + calculate_output_code(line));

    println!("{}", res);
}
