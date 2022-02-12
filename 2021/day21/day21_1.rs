use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut players_pos: Vec<usize> = data
        .split("\n")
        .map(|line| line.split(": ").collect::<Vec<&str>>()[1].parse().unwrap())
        .collect();

    let mut scores = [0, 0];
    let mut turn = 0;
    let res;

    loop {
        let dice_rolls = [
            ((turn * 3) % 100) + 1,
            (((turn * 3) + 1) % 100) + 1,
            (((turn * 3) + 2) % 100) + 1,
        ];

        players_pos[turn % 2] =
            ((players_pos[turn % 2] + dice_rolls[0] + dice_rolls[1] + dice_rolls[2] - 1) % 10) + 1;

        scores[turn % 2] += players_pos[turn % 2];

        if scores[turn % 2] >= 1000 {
            res = scores[(turn + 1) % 2] * (turn + 1) * 3;
            break;
        }

        turn += 1;
    }

    println!("{}", res);
}
