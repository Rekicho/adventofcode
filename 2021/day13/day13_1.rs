use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let parts: Vec<&str> = data.split("\n\n").collect();
    let mut coords: Vec<(u32, u32)> = parts[0]
        .split("\n")
        .map(|x| {
            let numbers = x
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<u32>>();
            (numbers[0], numbers[1])
        })
        .collect();
    let instr: Vec<(bool, u32)> = parts[1]
        .split("\n")
        .map(|x| {
            let instr_parts = x.split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .collect::<Vec<&str>>();
            (instr_parts[0] == "y", instr_parts[1].parse().unwrap())
        })
        .collect();

    for instruction in &instr[..1] {
        for (i, coord) in coords.clone().iter().enumerate() {
            if instruction.0 && coord.1 > instruction.1 {
                coords[i] = (coord.0, 2 * instruction.1 - coord.1)
            } else if !instruction.0 && coord.0 > instruction.1 {
                coords[i] = (2 * instruction.1 - coord.0, coord.1)
            }
        }
    }

    let unique: HashSet<(u32, u32)> = coords.into_iter().collect();

    println!("{}", unique.len());
}
