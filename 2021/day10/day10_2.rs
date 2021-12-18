use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<Vec<char>> = data
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut scores: Vec<u64> = Vec::new();

    'outer: for line in lines {
        let mut stack: Vec<char> = Vec::new();

        for delimiter in line {
            match delimiter {
                '(' | '[' | '{' | '<' => stack.push(delimiter),
                ')' => {
                    if *stack.last().unwrap() == '(' {
                        stack.pop();
                    } else {
                        continue 'outer;
                    }
                }
                ']' => {
                    if *stack.last().unwrap() == '[' {
                        stack.pop();
                    } else {
                        continue 'outer;
                    }
                }
                '}' => {
                    if *stack.last().unwrap() == '{' {
                        stack.pop();
                    } else {
                        continue 'outer;
                    }
                }
                '>' => {
                    if *stack.last().unwrap() == '<' {
                        stack.pop();
                    } else {
                        continue 'outer;
                    }
                }
                _ => (),
            }
        }

        stack.reverse();
        scores.push(stack.into_iter().fold(0, |acc, del| {
            acc * 5
                + match del {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                }
        }));
    }

    scores.sort();
    let res = scores[scores.len() / 2];

    println!("{}", res);
}
