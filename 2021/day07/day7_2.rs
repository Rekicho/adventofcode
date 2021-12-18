use std::fs;

fn fuel_cost(pos_diff: i32) -> i32 {
    // https://en.wikipedia.org/wiki/Arithmetic_progression
    pos_diff * (1 + pos_diff) / 2
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let numbers: Vec<i32> = data.split(",").map(|x| x.parse().unwrap()).collect();
    let min = numbers.clone().into_iter().min().unwrap();
    let max = numbers.clone().into_iter().max().unwrap();

    let res = (min..max)
        .map(|x| {
            numbers
                .clone()
                .into_iter()
                .fold(0, |acc, y| acc + fuel_cost((y - x).abs()))
        })
        .min()
        .unwrap();
    println!("{}", res);
}
