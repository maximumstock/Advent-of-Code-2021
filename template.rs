use std::collections::HashMap;

fn main() {
    let map = parse_input(include_str!("input.txt"));

    let part1 = solve(map);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 0);
}

fn solve(map: HashMap<(isize, isize), u32>) -> usize {
    todo!()
}

fn parse_input(input: &str) -> HashMap<(isize, isize), u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((row as isize, col as isize), c.to_digit(10).unwrap()))
        })
        .collect::<HashMap<_, _>>()
}
