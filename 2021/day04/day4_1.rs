use std::fs;

fn add_called_number(card: &mut Vec<Vec<(u32, bool)>>, called_number: u32) {
    for line in card {
        *line = line.into_iter().map(|(num, called)| (*num, *called || *num == called_number)).collect();
    }
}

fn check_win(card: Vec<Vec<(u32, bool)>>) -> bool {
    let completed_lines: usize = card.clone()
                                     .into_iter()
                                     .filter(|line| line.into_iter()
                                                        .filter(|(_, called)| *called == false)
                                                        .count() == 0)
                                     .count();
    let completed_cols: usize = (0..card.clone().len()).filter(|i| card.clone()
                                                                        .into_iter()
                                                                        .filter(|line| line[*i].1 == false)
                                                                        .count() == 0)
                                                       .count();

    completed_lines != 0 || completed_cols != 0
}

fn sum_uncalled(card: Vec<Vec<(u32, bool)>>) -> u32 {
    card.into_iter()
        .fold(0, |acc, line| acc + line.into_iter()
        .fold(0, |sum, (number, called)| if called { sum } else { sum + number }))
}


fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let info = data.split("\n\n");
    let mut called: Vec<u32> = Vec::new();
    let mut cards: Vec<Vec<Vec<(u32, bool)>>> = Vec::new();

    for (i, lines) in info.enumerate() {
        if i == 0 {
            called = lines.split(",").map(|x| x.parse().unwrap()).collect();
        } else {
            let mut card: Vec<Vec<(u32, bool)>> = Vec::new();
            for line in lines.split("\n") {
                card.push(line.split_whitespace().map(|x| (x.parse().unwrap(), false)).collect());
            }
            cards.push(card);
        }
    }

    let mut res = 0;

    'outer: for called_number in called {
        for card in &mut cards {
            add_called_number(card, called_number);
            if check_win(card.clone()) {
                res = sum_uncalled(card.clone()) * called_number;
                break 'outer;
            }
        }
    }

    println!("{}", res);
}