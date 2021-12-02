use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let steps = data.split("\n");
    let mut horizontal = 0;
    let mut depth = 0;

    for step in steps {
        let split: Vec<&str> = step.split(" ").collect();
        match split[0] {
            "forward" => horizontal += split[1].parse::<i32>().unwrap(),
            "down" => depth += split[1].parse::<i32>().unwrap(),
            "up" => depth -= split[1].parse::<i32>().unwrap(),
            _ => (),
        }
    }

    println!("{}", horizontal * depth);
}