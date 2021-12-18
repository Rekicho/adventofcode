use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let res = data.split("\n").fold(0, |acc, line| {
        acc + line.split(" | ").collect::<Vec<&str>>()[1]
            .split(" ")
            .fold(0, |line_acc, word| {
                if [2, 3, 4, 7].contains(&word.chars().count()) {
                    line_acc + 1
                } else {
                    line_acc
                }
            })
    });

    println!("{}", res);
}
