use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut numbers: Vec<i32> = data.split(",").map(|x| x.parse().unwrap()).collect();
    numbers.sort();

    // Not actually median if number.len() is even, but it does not matter in this case
    let median: i32 = numbers[numbers.len() / 2];

    let res = numbers.into_iter().fold(0, |acc, x| acc + (x - median).abs());
    println!("{}", res);
}