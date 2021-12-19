use std::cmp;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Number(u32),
    Other(char),
}

fn explode(pair: Vec<Element>) -> (Vec<Element>, bool) {
    let mut level = 0;
    let mut changed = false;
    let mut index = 0;

    for (i, elem) in pair.iter().enumerate() {
        if *elem == Element::Other('[') {
            level += 1;
        } else if *elem == Element::Other(']') {
            level -= 1;
        }
        if level == 5 {
            changed = true;
            index = i;
            break;
        }
    }

    if changed {
        let mut left = Vec::from(&pair[..index]);
        let mut right = Vec::from(&pair[index + 5..]);

        let mut left_value = 0;
        let mut right_value = 0;

        if let Element::Number(x) = &pair[index + 1] {
            left_value = *x;
        }
        if let Element::Number(x) = &pair[index + 3] {
            right_value = *x;
        }

        for (i, elem) in left.clone().into_iter().rev().enumerate() {
            if let Element::Number(x) = elem {
                left[index - i - 1] = Element::Number(left_value + x);
                break;
            }
        }

        for (i, elem) in right.clone().into_iter().enumerate() {
            if let Element::Number(x) = elem {
                right[i] = Element::Number(right_value + x);
                break;
            }
        }

        let mut res: Vec<Element> = Vec::new();
        res.extend(left.clone());
        res.push(Element::Number(0));
        res.extend(right.clone());

        return (res, true);
    }

    (pair, false)
}

fn split(pair: Vec<Element>) -> (Vec<Element>, bool) {
    let mut changed = false;
    let mut index = 0;

    for (i, elem) in pair.iter().enumerate() {
        if let Element::Number(x) = elem {
            if *x >= 10 {
                changed = true;
                index = i;
                break;
            }
        }
    }

    if changed {
        let left = Vec::from(&pair[..index]);
        let right = Vec::from(&pair[index + 1..]);

        let mut value = 0;

        if let Element::Number(x) = &pair[index] {
            value = *x;
        }

        let left_value = value / 2;
        let right_value = if value % 2 == 0 {
            left_value
        } else {
            left_value + 1
        };

        let mut res: Vec<Element> = Vec::new();
        res.extend(left.clone());
        res.extend(Vec::from([
            Element::Other('['),
            Element::Number(left_value),
            Element::Other(','),
            Element::Number(right_value),
            Element::Other(']'),
        ]));
        res.extend(right.clone());

        return (res, true);
    }

    (pair, false)
}

fn reduce(pair: Vec<Element>) -> Vec<Element> {
    let mut reduced = pair;

    loop {
        let mut res = explode(reduced.clone());

        if res.1 {
            reduced = res.0;
            continue;
        }

        res = split(reduced.clone());

        if !res.1 {
            break;
        }

        reduced = res.0;
    }

    reduced
}

fn add(left: Vec<Element>, right: Vec<Element>) -> Vec<Element> {
    let mut res: Vec<Element> = Vec::new();

    res.push(Element::Other('['));
    res.extend(left);
    res.push(Element::Other(','));
    res.extend(right);
    res.push(Element::Other(']'));

    reduce(res)
}

fn magnitude(pair: Vec<Element>) -> u32 {
    if let Element::Number(x) = &pair[0] {
        return *x;
    }

    let mut level = 0;
    let mut index = 0;

    for (i, elem) in pair.iter().enumerate() {
        if *elem == Element::Other('[') {
            level += 1;
        } else if *elem == Element::Other(']') {
            level -= 1;
        } else if *elem == Element::Other(',') && level == 1 {
            index = i;
            break;
        }
    }

    let left = Vec::from(&pair[1..index]);
    let right = Vec::from(&pair[index + 1..pair.len() - 1]);

    3 * magnitude(left) + 2 * magnitude(right)
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let pairs: Vec<Vec<Element>> = data
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch.is_digit(10) {
                        Element::Number(ch.to_digit(10).unwrap())
                    } else {
                        Element::Other(ch)
                    }
                })
                .collect::<Vec<Element>>()
        })
        .collect();

    let mut res = 0;

    for (i, first) in pairs.iter().enumerate() {
        for (j, second) in pairs.iter().enumerate() {
            if i != j {
                res = cmp::max(res, magnitude(add(first.to_vec(), second.to_vec())));
            }
        }
    }

    println!("{}", res);
}
