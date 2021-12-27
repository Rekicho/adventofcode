use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn align(current: &Vec<Vec<i32>>, candidate: &Vec<Vec<i32>>) -> (bool, Vec<Vec<i32>>) {
    let mut updated: Vec<Vec<i32>> = Vec::new();
    let possibilities = [(0, 1), (1, 1), (2, 1), (0, -1), (1, -1), (2, -1)];

    let mut matched_dimensions: Vec<u32> = Vec::new();

    'outer: for dimension in 0..3 {
        let dimension_current: Vec<i32> = current.iter().map(|coords| coords[dimension]).collect();

        for (dim, scale) in possibilities {
            if matched_dimensions.contains(&dim) {
                continue;
            }

            let transformed: Vec<i32> = candidate
                .iter()
                .map(|coord| coord[dim as usize] * scale)
                .collect();

            let mut diff: Vec<i32> = Vec::new();

            for i in dimension_current.iter() {
                for j in transformed.iter() {
                    diff.push(j - i);
                }
            }

            let mut diff_counts: HashMap<i32, i32> = HashMap::new();

            for x in diff {
                *diff_counts.entry(x).or_insert(0) += 1;
            }

            let max = diff_counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

            if *max.1 >= 12 {
                matched_dimensions.push(dim);
                updated.push(transformed.iter().map(|coord| coord - *max.0).collect());
                continue 'outer;
            }
        }

        return (false, Vec::new());
    }

    let mut res: Vec<Vec<i32>> = Vec::new();

    for i in 0..updated[0].len() {
        res.push(Vec::from([updated[0][i], updated[1][i], updated[2][i]]));
    }

    (true, res)
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let scanners: Vec<Vec<Vec<i32>>> = data
        .split("\n\n")
        .map(|line| {
            line.split("---\n").collect::<Vec<&str>>()[1]
                .split("\n")
                .map(|l| l.split(",").map(|coord| coord.parse().unwrap()).collect())
                .collect::<Vec<Vec<i32>>>()
        })
        .collect();

    let mut beacons: HashSet<Vec<i32>> = HashSet::new();
    let mut aligned: VecDeque<Vec<Vec<i32>>> = VecDeque::from([scanners[0].clone()]);
    let mut candidates: Vec<Vec<Vec<i32>>> = Vec::new();

    for i in 1..scanners.len() {
        candidates.push(scanners[i].clone());
    }

    while !aligned.is_empty() {
        let current = aligned.pop_front().unwrap();
        let mut not_aligned: Vec<Vec<Vec<i32>>> = Vec::new();

        for candidate in candidates {
            let (is_aligned, updated) = align(&current, &candidate);

            if is_aligned {
                aligned.push_back(updated);
            } else {
                not_aligned.push(candidate);
            }
        }

        candidates = not_aligned;
        beacons.extend(current);
    }

    println!("{:?}", beacons.len());
}
