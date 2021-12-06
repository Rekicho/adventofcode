use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut values: [u64; 9] = [0; 9];

    for number in data.split(",").map(|x| x.parse::<usize>().unwrap()) {
        values[number] += 1;
    }

    for _ in 0..256 {
        let old_values = values.clone();

        for i in 1..9 {
            values[i-1] = old_values[i];
        }

        values[8] = old_values[0];
        values[6] += old_values[0];
    }

    let res: u64 = values.iter().sum();
    println!("{}", res);
}