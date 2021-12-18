use std::fs;

fn parse_packet(binary: &[char]) -> (&[char], usize) {
    let mut version: usize =
        usize::from_str_radix(&binary[..3].into_iter().collect::<String>(), 2).unwrap();

    let type_id = &binary[3..6].into_iter().collect::<String>();

    if type_id == "100" {
        let mut index = 6;

        loop {
            let group_bit = binary[index];
            index += 5;

            if group_bit == '0' {
                break;
            }
        }

        (&binary[index..], version)
    } else {
        let lenght_type = binary[6];

        if lenght_type == '0' {
            let subpacket_length: usize =
                usize::from_str_radix(&binary[7..22].into_iter().collect::<String>(), 2).unwrap();

            let mut subpackets = &binary[22..22 + subpacket_length];

            while subpackets.len() != 0 {
                let res = parse_packet(&subpackets);

                subpackets = res.0;
                version += res.1;
            }

            (&binary[22 + subpacket_length..], version)
        } else {
            let num_subpackets: usize =
                usize::from_str_radix(&binary[7..18].into_iter().collect::<String>(), 2).unwrap();

            let mut subpackets = &binary[18..];

            for _ in 0..num_subpackets {
                let res = parse_packet(&subpackets);

                subpackets = res.0;
                version += res.1;
            }

            (subpackets, version)
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
