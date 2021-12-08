use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let lines = data.split("\n");
    let mut map: HashMap<(i32,i32), i32> = HashMap::new();

    for line in lines {
        let ends = line.split(" -> ").collect::<Vec<&str>>();
        let from: Vec<i32> = ends[0].split(",").map(|x| x.parse().unwrap()).collect();
        let to: Vec<i32> = ends[1].split(",").map(|x| x.parse().unwrap()).collect();
        let mut m: i32 = 0;
        let mut remainder: i32 = 0;
        if from[0] - to[0] != 0 {
            m = ((from[1] - to[1]) / (from[0] - to[0])).abs();
            remainder = ((from[1] - to[1]) % (from[0] - to[0])).abs();
        }

        let mut points: Vec<(i32, i32)> = Vec::new();

        if from[0] == to[0] {
            let min = cmp::min(from[1], to[1]);
            let max = cmp::max(from[1], to[1]);

            for y in min..max+1 {
                points.push((from[0], y));
            }
        } else if from[1] == to[1] {
            let min = cmp::min(from[0], to[0]);
            let max = cmp::max(from[0], to[0]);
            for x in min..max+1 {
                points.push((x, from[1]));
            }
        } else if from[0] == from[1] && to[0] == to[1] {
            let min = cmp::min(from[0], to[0]);
            let max = cmp::max(from[0], to[0]);

            for value in min..max+1 {
                points.push((value, value));
            }
        } else if m == 1 && remainder == 0 {
            let steps = (to[0] - from[0]).abs();
            let x_inc = (to[0] - from[0]) / (to[0] - from[0]).abs();
            let y_inc = (to[1] - from[1]) / (to[1] - from[1]).abs();

            for i in 0..steps+1 {
                points.push((from[0] + i * x_inc, from[1] + i * y_inc));
            }
        }

        for point in points {
            if map.contains_key(&point) {
                map.insert(point, map[&point] + 1);
            } else {
                map.insert(point, 1);
            }
        }
    }

    let res = map.into_iter().filter(|&(_ ,v)| v >= 2).count();
    println!("{}", res);
}