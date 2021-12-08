use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut numbers: Vec<u32> = data.split(",").map(|x| x.parse().unwrap()).collect();

    for _ in 0..80 {
        for i in 0..numbers.len() {
            if numbers[i] == 0 {
                numbers[i] = 6;
                numbers.push(8);
            } else {
                numbers[i] -= 1;
            }
        }
    }

    println!("{}", numbers.len());
}