use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in data.split("\n") {
        let parts: Vec<&str> = line.split("-").collect();
        let src: &str = parts[0];
        let dest: &str = parts[1];

        if graph.contains_key(src) {
            let mut dests = graph.get(src).unwrap().clone();
            dests.push(dest);
            graph.insert(src, dests);
        } else {
            let dests = Vec::from([dest]);
            graph.insert(src, dests);
        }

        if graph.contains_key(dest) {
            let mut srcs = graph.get(dest).unwrap().clone();
            srcs.push(src);
            graph.insert(dest, srcs);
        } else {
            let srcs = Vec::from([src]);
            graph.insert(dest, srcs);
        }
    }

    let mut paths: Vec<Vec<&str>> = Vec::new();
    let mut stack: VecDeque<(Vec<&str>, &str)> = VecDeque::new();

    stack.push_back((Vec::new(), "start"));

    while !stack.is_empty() {
        let (mut current_path, current_node) = stack.pop_front().unwrap().clone();

        match current_node {
            "start" => {
                if current_path.len() == 0 {
                    let nodes_to_check = graph.get("start").unwrap().clone();

                    for node in nodes_to_check {
                        let mut new_path = current_path.clone();
                        new_path.push(current_node);

                        stack.push_back((new_path, node));
                    }
                }
            }
            "end" => {
                current_path.push("end");
                paths.push(current_path)
            }
            _ => {
                if !current_path.contains(&current_node)
                    || current_node.chars().all(char::is_uppercase)
                {
                    let nodes_to_check = graph.get(current_node).unwrap().clone();

                    for node in nodes_to_check {
                        let mut new_path = current_path.clone();
                        new_path.push(current_node);

                        stack.push_back((new_path, node));
                    }
                }
            }
        }
    }

    println!("{}", paths.len());
}
