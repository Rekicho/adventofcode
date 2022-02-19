use std::cmp;
use std::fs;

fn colision_range(range1: [isize; 2], range2: [isize; 2]) -> Result<[isize; 2], &'static str> {
    if range1[1] < range2[0] || range1[0] > range2[1] {
        return Err("");
    }

    let min = cmp::min(cmp::max(range1[0], range2[0]), range2[1]);
    let max = cmp::min(cmp::max(range1[1], range2[0]), range2[1]);

    Ok([min, max])
}

fn count_zones(
    instruction: (bool, [[isize; 2]; 3]),
    others: Vec<(bool, [[isize; 2]; 3])>,
) -> isize {
    let x_range = instruction.1[0];
    let y_range = instruction.1[1];
    let z_range = instruction.1[2];

    let mut res = (x_range[1] - x_range[0] + 1)
        * (y_range[1] - y_range[0] + 1)
        * (z_range[1] - z_range[0] + 1);

    let mut collisions: Vec<(bool, [[isize; 2]; 3])> = Vec::new();

    for other in others {
        let x_range_other = other.1[0];
        let y_range_other = other.1[1];
        let z_range_other = other.1[2];

        let colision_x = colision_range(x_range, x_range_other);
        let colision_y = colision_range(y_range, y_range_other);
        let colision_z = colision_range(z_range, z_range_other);

        match (colision_x, colision_y, colision_z) {
            (Ok(col_x), Ok(col_y), Ok(col_z)) => collisions.push((other.0, [col_x, col_y, col_z])),
            _ => (),
        }
    }

    for (i, collision) in collisions.iter().enumerate() {
        res -= count_zones(*collision, (&collisions[i + 1..]).to_vec())
    }

    res
}

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

    let mut res = 0;

    for (i, instruction) in instructions.iter().enumerate() {
        if !instruction.0 {
            continue;
        }

        res += count_zones(*instruction, (&instructions[i + 1..]).to_vec());
    }

    println!("{}", res);
}
