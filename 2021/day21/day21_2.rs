use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn sim_game(
    results: &mut HashMap<[usize; 4], [usize; 2]>,
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
) -> [usize; 2] {
    match results.get(&[p1_pos, p2_pos, p1_score, p2_score]) {
        Some(res) => return *res,
        _ => (),
    }

    let mut res = [0, 0];

    if p1_score >= 21 {
        res = [1, 0];
    } else if p2_score >= 21 {
        res = [0, 1];
    } else {
        for first_roll in 1..4 {
            for second_roll in 1..4 {
                for third_roll in 1..4 {
                    let new_p1_pos =
                        ((p1_pos + first_roll + second_roll + third_roll - 1) % 10) + 1;
                    let new_p1_score = p1_score + new_p1_pos;

                    let new_res = sim_game(results, p2_pos, new_p1_pos, p2_score, new_p1_score);
                    res[0] += new_res[1];
                    res[1] += new_res[0];
                }
            }
        }
    }

    results.insert([p1_pos, p2_pos, p1_score, p2_score], res);

    res
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let players_pos: Vec<usize> = data
        .split("\n")
        .map(|line| line.split(": ").collect::<Vec<&str>>()[1].parse().unwrap())
        .collect();

    let mut results: HashMap<[usize; 4], [usize; 2]> = HashMap::new();

    let wins = sim_game(&mut results, players_pos[0], players_pos[1], 0, 0);
    let res = max(wins[0], wins[1]);

    println!("{}", res);
}
