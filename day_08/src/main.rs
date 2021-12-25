use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = parse_input(include_str!("input.txt"));

    let part1 = part1(&input);
    assert_eq!(part1, 470);
    println!("Part 1: {}", part1);

    let part2 = part2(input);
    assert_eq!(part2, 989396);
    println!("Part 2: {}", part2);
}

fn part1(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|l| {
            l.outputs
                .iter()
                .filter(|o| o.len() == 2 || o.len() == 4 || o.len() == 3 || o.len() == 7)
                .count()
        })
        .sum()
}

fn part2(lines: Vec<Line>) -> usize {
    lines.into_iter().map(|mut l| l.solve()).sum()
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

struct Line {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Line {
    fn solve(&mut self) -> usize {
        self.inputs.sort_by_key(|a| a.len());

        let one = self.inputs.get(0).unwrap();
        let seven = self.inputs.get(1).unwrap();
        let four = self.inputs.get(2).unwrap();
        let eight = self.inputs.get(9).unwrap();

        let fives = self
            .inputs
            .iter()
            .filter(|i| i.len() == 5)
            .collect::<Vec<_>>();

        let sixes = self
            .inputs
            .iter()
            .filter(|i| i.len() == 6)
            .collect::<Vec<_>>();

        let three = fives
            .iter()
            .find(|s| one.chars().all(|c| s.contains(c)))
            .unwrap();

        let nine = sixes
            .iter()
            .find(|s| seven.chars().chain(four.chars()).all(|c| s.contains(c)))
            .unwrap();

        let six = sixes
            .iter()
            .find(|s| s.ne(&nine) && !one.chars().all(|c| s.contains(c)))
            .unwrap();

        let zero = sixes.iter().find(|s| s.ne(&nine) && s.ne(&six)).unwrap();

        let five = fives
            .iter()
            .find(|s| left_str_minus_right_str(six, s).len() == 1)
            .unwrap();

        let two = fives.iter().find(|s| s.ne(&three) && s.ne(&five)).unwrap();

        let mut mapping: HashMap<&String, usize> = HashMap::new();
        mapping.insert(zero, 0);
        mapping.insert(one, 1);
        mapping.insert(two, 2);
        mapping.insert(three, 3);
        mapping.insert(four, 4);
        mapping.insert(five, 5);
        mapping.insert(six, 6);
        mapping.insert(seven, 7);
        mapping.insert(eight, 8);
        mapping.insert(nine, 9);

        self.outputs
            .iter()
            .flat_map(|o| mapping.get(o))
            .fold(0, |acc, next| acc * 10 + next)
    }
}

fn left_str_minus_right_str(a: &str, b: &str) -> Vec<char> {
    a.chars()
        .filter(|ca| !b.chars().any(|cb| cb == *ca))
        .collect::<Vec<_>>()
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" | ");
        let inputs = parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(sort_str)
            .collect::<Vec<String>>();
        let outputs = parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(sort_str)
            .collect::<Vec<String>>();

        Ok(Self { inputs, outputs })
    }
}

fn sort_str(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<_>>();
    chars.sort_unstable();
    chars.iter().collect::<String>()
}
