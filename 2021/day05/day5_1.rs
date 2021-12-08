use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let lines = data.split("\n");
    let mut map: HashMap<(u32,u32), u32> = HashMap::new();

    for line in lines {
        let ends = line.split(" -> ").collect::<Vec<&str>>();
        let from: Vec<u32> = ends[0].split(",").map(|x| x.parse().unwrap()).collect();
        let to: Vec<u32> = ends[1].split(",").map(|x| x.parse().unwrap()).collect();

        let mut points: Vec<(u32, u32)> = Vec::new();

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