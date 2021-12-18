use std::cmp;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let parts: Vec<&str> = data.split(": ").collect::<Vec<&str>>()[1]
        .split(", ")
        .collect();

    let x: Vec<i32> = parts[0].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .map(|num| num.parse().unwrap())
        .collect();

    let y: Vec<i32> = parts[1].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .map(|num| num.parse().unwrap())
        .collect();

    let xmin = x[0];
    let xmax = x[1];
    let ymin = y[0];
    let ymax = y[1];

    let mut possible: Vec<(i32, i32)> = Vec::new();

    for vxi in 0..xmax + 1 {
        for vyi in ymin..-ymin + 1 {
            let mut xt = 0;
            let mut yt = 0;

            let mut vxt = vxi;
            let mut vyt = vyi;

            while xt <= xmax && yt >= ymin {
                if xt >= xmin && yt <= ymax {
                    possible.push((xt, yt));
                    break;
                }

                xt = xt + vxt;
                yt = yt + vyt;

                vxt = cmp::max(vxt - 1, 0);
                vyt -= 1;
            }
        }
    }

    println!("{:?}", possible.len());
}
