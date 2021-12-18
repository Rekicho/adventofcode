use std::collections::VecDeque;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut octopuses: [[(u32, bool); 12]; 12] = [[(0, false); 12]; 12];

    for (i, line) in data.split("\n").enumerate() {
        for (j, octopus) in line.chars().enumerate() {
            octopuses[i + 1][j + 1] = (octopus.to_digit(10).unwrap(), false);
        }
    }

    let mut flashes: u32 = 0;

    for _ in 0..100 {
        for i in 1..11 {
            for j in 1..11 {
                if octopuses[i][j].1 {
                    octopuses[i][j] = (1, false);
                } else {
                    octopuses[i][j] = (octopuses[i][j].0 + 1, false);
                }
            }
        }

        for i in 1..11 {
            for j in 1..11 {
                if octopuses[i][j].1 {
                    continue;
                }

                if octopuses[i][j].0 > 9 {
                    let mut to_flash: VecDeque<(usize, usize)> = VecDeque::new();
                    octopuses[i][j].1 = true;
                    to_flash.push_back((i, j));

                    while !to_flash.is_empty() {
                        flashes += 1;
                        let (ci, cj) = to_flash.pop_front().unwrap();

                        let adjacent: [(usize, usize); 8] = [
                            (ci - 1, cj - 1),
                            (ci - 1, cj),
                            (ci - 1, cj + 1),
                            (ci, cj - 1),
                            (ci, cj + 1),
                            (ci + 1, cj - 1),
                            (ci + 1, cj),
                            (ci + 1, cj + 1),
                        ];

                        for (ii, jj) in adjacent {
                            if ii < 1 || ii > 10 || jj < 1 || jj > 10 || octopuses[ii][jj].1 {
                                continue;
                            }

                            octopuses[ii][jj].0 += 1;

                            if octopuses[ii][jj].0 > 9 {
                                octopuses[ii][jj].1 = true;
                                to_flash.push_back((ii, jj));
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", flashes);
}
