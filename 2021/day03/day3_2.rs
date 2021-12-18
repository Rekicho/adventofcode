use std::convert::TryInto;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let numbers = data.split("\n");
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for number in numbers {
        matrix.push(number.chars().map(|x| x.to_digit(2).unwrap()).collect());
    }

    let mut generator_matrix = matrix.clone();
    let mut i = 0;

    while generator_matrix.len() > 1 {
        let sum = generator_matrix
            .clone()
            .into_iter()
            .fold(0, |acc, x| acc + x[i]);

        if sum * 2 >= generator_matrix.len().try_into().unwrap() {
            generator_matrix = generator_matrix.into_iter().filter(|x| x[i] == 1).collect();
        } else {
            generator_matrix = generator_matrix.into_iter().filter(|x| x[i] == 0).collect();
        }

        i += 1;
    }

    let mut scrubber_matrix = matrix.clone();
    i = 0;

    while scrubber_matrix.len() > 1 {
        let sum = scrubber_matrix
            .clone()
            .into_iter()
            .fold(0, |acc, x| acc + x[i]);

        if sum * 2 < scrubber_matrix.len().try_into().unwrap() {
            scrubber_matrix = scrubber_matrix.into_iter().filter(|x| x[i] == 1).collect();
        } else {
            scrubber_matrix = scrubber_matrix.into_iter().filter(|x| x[i] == 0).collect();
        }

        i += 1;
    }

    let res = scrubber_matrix[0]
        .clone()
        .into_iter()
        .fold(0, |acc, x| acc * 2 + x)
        * generator_matrix[0]
            .clone()
            .into_iter()
            .fold(0, |acc, x| acc * 2 + x);

    println!("{}", res);
}
