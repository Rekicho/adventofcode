use std::collections::VecDeque;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut map: Vec<Vec<(u32, bool)>> = data
        .split("\n")
        .map(|line| {
            Vec::<(u32, bool)>::from([(10, true)])
                .into_iter()
                .chain(
                    line.chars()
                        .map(|x| (x.to_digit(10).unwrap(), false))
                        .collect::<Vec<(u32, bool)>>(),
                )
                .chain(Vec::<(u32, bool)>::from([(10, true)]).into_iter())
                .collect()
        })
        .collect();

    let mut empty: Vec<(u32, bool)> = Vec::new();
    empty.resize(map[0].len(), (10, true));
    map.insert(0, empty.clone());
    map.insert(map.len(), empty);

    let mut basins: Vec<u32> = Vec::new();

    for (i, line) in map.clone().into_iter().enumerate() {
        if i == 0 || i == map.len() - 1 {
            continue;
        }

        for (j, height) in line.clone().into_iter().enumerate() {
            if height.1 {
                continue;
            }
            map[i][j].1 = true;
            if height.0 == 9 {
                continue;
            }

            let mut len: u32 = 1;
            let mut possible: VecDeque<(usize, usize)> = VecDeque::new();

            possible.push_back((i - 1, j));
            possible.push_back((i, j - 1));
            possible.push_back((i, j + 1));
            possible.push_back((i + 1, j));

            while !possible.is_empty() {
                let current = possible.pop_front().unwrap();
                let current_height = map[current.0][current.1];

                if current_height.1 {
                    continue;
                }
                map[current.0][current.1].1 = true;
                if current_height.0 == 9 {
                    continue;
                }

                len += 1;
                possible.push_back((current.0 - 1, current.1));
                possible.push_back((current.0, current.1 - 1));
                possible.push_back((current.0, current.1 + 1));
                possible.push_back((current.0 + 1, current.1));
            }

            basins.push(len);
        }
    }

    basins.sort_by(|a, b| b.cmp(a));
    let res = basins.into_iter().take(3).fold(1, |acc, x| acc * x);
    println!("{}", res);
}
