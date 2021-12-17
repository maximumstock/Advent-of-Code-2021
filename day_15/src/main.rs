use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    let map = parse_input(include_str!("input.txt"));
    let part1 = solve(map, (0, 0), (99, 99));
    assert_eq!(part1, 508);
    println!("Part 1: {}", part1);

    let extended_map = parse_extended_map(include_str!("input.txt"), 100);
    assert_eq!(extended_map.len(), 250_000);
    let part2 = solve(extended_map, (0, 0), (499, 499));
    assert_eq!(part2, 2872);
    println!("Part 2: {}", part2);
}

type Point = (isize, isize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    risk: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(map: HashMap<Point, usize>, start: Point, end: Point) -> usize {
    let mut distances = HashMap::<Point, usize>::new();
    let mut heap = BinaryHeap::new();

    for &point in map.keys() {
        distances.insert(point, usize::MAX);
    }
    distances.insert(start, 0);
    heap.push(State {
        position: start,
        risk: 0,
    });

    while let Some(State { position, risk }) = heap.pop() {
        if position == end {
            return risk;
        }

        if risk > distances[&position] {
            continue;
        }

        for (neighbour, neighbour_risk) in neighbours(&map, position) {
            let next = State {
                position: neighbour,
                risk: risk + neighbour_risk,
            };

            if next.risk < distances[&neighbour] {
                heap.push(next);
                distances.insert(neighbour, next.risk);
            }
        }
    }

    panic!("asdf")
}

fn neighbours(map: &HashMap<Point, usize>, current: Point) -> Vec<(Point, usize)> {
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .filter_map(|pos| {
            let new_pos = (current.0 + pos.0, current.1 + pos.1);
            map.get(&new_pos).map(|risk| (new_pos, *risk))
        })
        .collect()
}

fn parse_input(input: &str) -> HashMap<Point, usize> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim().chars().enumerate().map(move |(col, c)| {
                (
                    (col as isize, row as isize),
                    c.to_digit(10).unwrap() as usize,
                )
            })
        })
        .collect::<HashMap<_, _>>()
}

fn parse_extended_map(input: &str, dim: isize) -> HashMap<Point, usize> {
    let mut map = parse_input(input);

    for row in 0..5 {
        for col in 0..5 {
            if row == 0 && col == 0 {
                continue;
            }

            extend_map(&mut map, dim, row, col);
        }
    }

    map
}

fn extend_map(map: &mut HashMap<Point, usize>, dim: isize, row: isize, col: isize) {
    for y in 0..dim {
        for x in 0..dim {
            let mut new_value = map[&(x, y)] + row as usize + col as usize;
            if new_value > 9 {
                new_value %= 10;
                new_value += 1;
            }
            let new_x = x + col * dim;
            let new_y = y + row * dim;
            map.insert((new_x, new_y), new_value);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_extended_map, parse_input};

    #[test]
    fn test_map_extension() {
        let extended = parse_extended_map(include_str!("test.input.txt"), 10);
        let big = parse_input(include_str!("test.input.extended.txt"));

        assert_eq!(extended, big);
    }

    #[test]
    fn test_map_extension_small() {
        let extended = parse_extended_map("8", 1);
        let big = parse_input("89123\n91234\n12345\n23456\n34567");

        assert_eq!(extended[&(3, 4)], 6);
        assert_eq!(extended[&(4, 4)], 7);
        assert_eq!(extended, big);
    }
}
