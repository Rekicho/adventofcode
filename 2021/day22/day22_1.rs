use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<(bool, [[isize; 2]; 3])> = data
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let on = parts[0] == "on";
            let coords_parts: Vec<[isize; 2]> = parts[1]
                .split(",")
                .map(|coords_part| {
                    let coords: Vec<isize> = coords_part.split("=").collect::<Vec<&str>>()[1]
                        .split("..")
                        .map(|coord| coord.parse().unwrap())
                        .collect();
                    [coords[0], coords[1]]
                })
                .collect();

            (on, [coords_parts[0], coords_parts[1], coords_parts[2]])
        })
        .collect();

    let mut active = HashSet::<[isize; 3]>::new();

    for instruction in instructions {
        for x in max(instruction.1[0][0], -50)..min(instruction.1[0][1], 50) + 1 {
            for y in max(instruction.1[1][0], -50)..min(instruction.1[1][1], 50) + 1 {
                for z in max(instruction.1[2][0], -50)..min(instruction.1[2][1], 50) + 1 {
                    if instruction.0 {
                        active.insert([x, y, z]);
                    } else {
                        active.remove(&[x, y, z]);
                    }
                }
            }
        }
    }

    let res = active.len();

    println!("{}", res);
}
