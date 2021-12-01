use std::fs;
use std::convert::TryFrom;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let depths: Vec<i32> = data.split("\n").map(|s| s.parse().unwrap()).collect();

    let res = depths[..].windows(2).flat_map(<&[i32; 2]>::try_from).filter(|[a, b]| a < b).count();
    println!("{}", res);
}