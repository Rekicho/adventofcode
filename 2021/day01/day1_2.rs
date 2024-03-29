use std::convert::TryFrom;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let depths: Vec<i32> = data.split("\n").map(|s| s.parse().unwrap()).collect();

    let res = depths[..]
        .windows(4)
        .flat_map(<&[i32; 4]>::try_from)
        .filter(|[a, _, _, b]| a < b)
        .count();
    println!("{}", res);
}
