use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let steps = data.split("\n");
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for step in steps {
        let split: Vec<&str> = step.split(" ").collect();
        match split[0] {
            "forward" => {
                horizontal += split[1].parse::<i32>().unwrap();
                depth += aim * split[1].parse::<i32>().unwrap()
            },
            "down" => aim += split[1].parse::<i32>().unwrap(),
            "up" => aim -= split[1].parse::<i32>().unwrap(),
            _ => (),
        }
    }

    println!("{}", horizontal * depth);
}