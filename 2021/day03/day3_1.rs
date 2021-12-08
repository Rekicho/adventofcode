use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let numbers = data.split("\n");
    let mut bit_sum = Vec::new();
    let mut count = 0;

    for number in numbers {
        for (i, bit) in number.chars().enumerate() {
            let bit_value = bit.to_digit(2).unwrap();
            if i >= bit_sum.len() {
                bit_sum.push(bit_value);
            } else {
                bit_sum[i] += bit_value;
            }
        }

        count += 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    // Could also XOR gamma with 111111... to get epsilon
    for bit in &bit_sum {
        gamma *= 2;
        epsilon *= 2;

        if bit * 2 > count {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("{:?}", gamma * epsilon);
}