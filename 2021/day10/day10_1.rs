use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<Vec<char>> = data.split("\n").map(|line| line.chars().collect()).collect();

    let mut res: u32 = 0;

    for line in lines {
        let mut stack: Vec<char> = Vec::new();

        for delimiter in line {
            match delimiter {
                '(' | '[' | '{' | '<' => stack.push(delimiter),
                ')' => if *stack.last().unwrap() == '(' { stack.pop(); } else { res += 3; break; },
                ']' => if *stack.last().unwrap() == '[' { stack.pop(); } else { res += 57; break; },
                '}' => if *stack.last().unwrap() == '{' { stack.pop(); } else { res += 1197; break; },
                '>' => if *stack.last().unwrap() == '<' { stack.pop(); } else { res += 25137; break; },
                _ => (),
            }
        }
    }

    println!("{}", res);
}