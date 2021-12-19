fn main() {
    let mut raw_packet = parse_input(include_str!("input.txt"));
    let root_packet = parse_packet(&mut raw_packet);

    let part1 = {
        let packet = root_packet.clone();
        sum_version_numbers(&packet)
    };
    println!("Part 1: {}", part1);
    assert_eq!(part1, 908);

    let part2 = evaluate_packet(&root_packet);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 10626195124371);
}

fn sum_version_numbers(root_packet: &Packet) -> usize {
    match root_packet {
        Packet::Literal(l) => l.version as usize,
        Packet::Operation(op) => {
            op.subpackets
                .iter()
                .map(|p| sum_version_numbers(p))
                .sum::<usize>()
                + op.version as usize
        }
    }
}

fn evaluate_packet(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(l) => l.literal,
        Packet::Operation(op) => {
            let mut it = op.subpackets.iter().map(|subp| evaluate_packet(subp));
            match op.type_id {
                0 => it.sum::<usize>(),
                1 => it.product::<usize>(),
                2 => it.min().unwrap(),
                3 => it.max().unwrap(),
                5 => {
                    let f = it.next().unwrap();
                    let s = it.next().unwrap();

                    if f > s {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let f = it.next().unwrap();
                    let s = it.next().unwrap();
                    if f < s {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let f = it.next().unwrap();
                    let s = it.next().unwrap();
                    if f == s {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Operation(Operation),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Operation {
    version: u32,
    type_id: u32,
    subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Literal {
    version: u32,
    type_id: u32,
    literal: usize,
}

fn parse_packet(mut raw_packet: &mut Vec<char>) -> Packet {
    let version = take_n(&mut raw_packet, 3);
    let type_id = take_n(&mut raw_packet, 3);
    if type_id == 4 {
        // literal value packet
        let literal = parse_literal(&mut raw_packet);
        Packet::Literal(Literal {
            version,
            type_id,
            literal,
        })
    } else {
        // operator packet
        let length_type_id = take_n(&mut raw_packet, 1);
        if length_type_id == 0 {
            // parse total length operator packet
            let n_bits = take_n(&mut raw_packet, 15);
            let packets = parse_n_bits(&mut raw_packet, n_bits);
            Packet::Operation(Operation {
                version,
                type_id,
                subpackets: packets,
            })
        } else {
            // parse total sub-packets operator packet
            let n_packets = take_n(&mut raw_packet, 11);
            let packets = parse_n_packets(&mut raw_packet, n_packets);
            Packet::Operation(Operation {
                version,
                type_id,
                subpackets: packets,
            })
        }
    }
}

fn parse_input(input: &str) -> Vec<char> {
    input
        .trim()
        .chars()
        .flat_map(|ch| {
            let v = ch.to_digit(16).unwrap();
            let s = format!("{:04b}", v);
            s.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<char>>()
}

fn take_n(raw: &mut Vec<char>, n: usize) -> u32 {
    let next = raw.drain(0..n).collect::<String>();
    u32::from_str_radix(&next, 2).unwrap()
}

fn parse_literal(raw: &mut Vec<char>) -> usize {
    let mut bits = vec![];

    while raw[0] != '0' {
        raw.drain(0..5).skip(1).for_each(|bit| bits.push(bit));
    }

    raw.drain(0..5).skip(1).for_each(|bit| bits.push(bit));

    let next = bits.iter().collect::<String>();
    usize::from_str_radix(&next, 2).unwrap()
}

fn parse_n_packets(raw: &mut Vec<char>, n_packets: u32) -> Vec<Packet> {
    let mut counter = 0;
    let mut packets = vec![];

    while counter != n_packets {
        let p = parse_packet(raw);
        packets.push(p);
        counter += 1;
    }

    packets
}

fn parse_n_bits(raw: &mut Vec<char>, n_bits: u32) -> Vec<Packet> {
    let start_len = raw.len();
    let mut packets = vec![];

    loop {
        let p = parse_packet(raw);
        packets.push(p);

        if start_len - raw.len() == n_bits as usize {
            break;
        }
    }

    packets
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, parse_packet, sum_version_numbers, Literal, Operation, Packet};

    #[test]
    fn test_parse_packet() {
        let mut raw = parse_input("38006F45291200");
        let packet = parse_packet(&mut raw);

        assert_eq!(
            packet,
            Packet::Operation(Operation {
                version: 1,
                type_id: 6,
                subpackets: vec![
                    Packet::Literal(Literal {
                        version: 6,
                        type_id: 4,
                        literal: 10,
                    },),
                    Packet::Literal(Literal {
                        version: 2,
                        type_id: 4,
                        literal: 20,
                    },),
                ],
            })
        );
    }

    #[test]
    fn test_sum_version_numbers() {
        let mut raw = parse_input("8A004A801A8002F478");
        let packet = parse_packet(&mut raw);
        assert_eq!(sum_version_numbers(&packet), 16);
    }
}
