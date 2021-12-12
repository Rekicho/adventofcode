use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut map: Vec<Vec<u32>> = data.split("\n")
                                     .map(|line| Vec::<u32>::from([10]).into_iter()
                                                                       .chain(line.chars()
                                                                                  .map(|x| x.to_digit(10).unwrap())
                                                                                  .collect::<Vec<u32>>())
                                                                       .chain(Vec::<u32>::from([10]).into_iter())
                                                                       .collect())
                                     .collect();

    let mut empty: Vec<u32> = Vec::new();
    empty.resize(map[0].len(), 10);
    map.insert(0, empty.clone());
    map.insert(map.len(), empty);

    let mut res: u32 = 0;

    for (i, line) in map.clone().into_iter().enumerate() {
        if i == 0 || i == map.len() - 1 {
            continue;
        }

        for (j, height) in line.clone().into_iter().enumerate() {
            if j == 0 || j == line.len() - 1 {
                continue;
            }

            let adjacent: [u32; 4] = [map[i-1][j], map[i][j-1], map[i][j+1], map[i+1][j]];

            if adjacent.iter().all(|x| x > &height) {
                res += height + 1;
            }
        }
    }

    println!("{}", res);
}