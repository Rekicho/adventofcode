use std::fs;

fn parse_packet(binary: &[char]) -> (&[char], usize) {
    let type_id: usize =
        usize::from_str_radix(&binary[3..6].into_iter().collect::<String>(), 2).unwrap();

    if type_id == 4 {
        let mut index = 6;
        let mut value = 0;

        loop {
            let new_value = usize::from_str_radix(
                &binary[index + 1..index + 5].into_iter().collect::<String>(),
                2,
            )
            .unwrap();
            value = value * 16 + new_value;

            let group_bit = binary[index];
            index += 5;
            if group_bit == '0' {
                break;
            }
        }

        (&binary[index..], value)
    } else {
        let lenght_type = binary[6];
        let new_subpackets;
        let mut subpackets_values = Vec::new();

        if lenght_type == '0' {
            let subpacket_length: usize =
                usize::from_str_radix(&binary[7..22].into_iter().collect::<String>(), 2).unwrap();

            let mut subpackets = &binary[22..22 + subpacket_length];
            new_subpackets = &binary[22 + subpacket_length..];

            while subpackets.len() != 0 {
                let res = parse_packet(&subpackets);

                subpackets = res.0;
                subpackets_values.push(res.1);
            }
        } else {
            let num_subpackets: usize =
                usize::from_str_radix(&binary[7..18].into_iter().collect::<String>(), 2).unwrap();

            let mut subpackets = &binary[18..];

            for _ in 0..num_subpackets {
                let res = parse_packet(&subpackets);

                subpackets = res.0;
                subpackets_values.push(res.1);
            }

            new_subpackets = subpackets;
        }

        match type_id {
            0 => (new_subpackets, subpackets_values.into_iter().sum()),
            1 => (new_subpackets, subpackets_values.into_iter().product()),
            2 => (new_subpackets, subpackets_values.into_iter().min().unwrap()),
            3 => (new_subpackets, subpackets_values.into_iter().max().unwrap()),
            5 => (
                new_subpackets,
                if subpackets_values[0] > subpackets_values[1] {
                    1
                } else {
                    0
                },
            ),
            6 => (
                new_subpackets,
                if subpackets_values[0] < subpackets_values[1] {
                    1
                } else {
                    0
                },
            ),
            7 => (
                new_subpackets,
                if subpackets_values[0] == subpackets_values[1] {
                    1
                } else {
                    0
                },
            ),
            _ => (new_subpackets, 0),
        }
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let binary: Vec<char> = data
        .chars()
        .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
        .collect::<Vec<String>>()
        .join("")
        .chars()
        .collect();

    let res = parse_packet(&binary).1;
    println!("{:?}", res);
}
