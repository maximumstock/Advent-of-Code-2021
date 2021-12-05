use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");

    let (part1, part2) = solve(input);
    assert_eq!(part1, 63552);
    println!("Final Score Part 1: {}", part1);
    assert_eq!(part2, 9020);
    println!("Final Score Part 2: {}", part2);
}

fn solve(input: &str) -> (u32, u32) {
    let (numbers, mut boards) = parse_input(input);
    let mut winners = Vec::new();

    for n in numbers {
        let mut to_remove = Vec::new();

        for (i, b) in boards.iter_mut().enumerate() {
            b.mark(n);
            if b.is_winner() {
                winners.push((b.score(), n));
                to_remove.push(i);
            }
        }

        while let Some(i) = to_remove.pop() {
            boards.remove(i);
        }
    }

    let (b1, n1) = winners.first().unwrap();
    let score_part1 = b1 * n1;

    let (b2, n2) = winners.last().unwrap();
    let score_part2 = b2 * n2;

    (score_part1, score_part2)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut blocks = input.split("\n\n");

    let numbers = blocks
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = blocks
        .map(|b| Board::from_str(b).unwrap())
        .collect::<Vec<Board>>();

    (numbers, boards)
}

struct Board {
    numbers: HashMap<u32, (usize, usize)>,
    positions: HashMap<(usize, usize), u32>,
    marks: HashMap<(usize, usize), bool>,
}

impl Board {
    fn mark(&mut self, number: u32) {
        if let Some((row, col)) = self.numbers.get(&number) {
            self.marks.insert((*row, *col), true);
        }
    }

    fn is_winner(&self) -> bool {
        (0..5).any(|row| (0..5).all(move |col| *self.marks.get(&(row, col)).unwrap()))
            || (0..5).any(|col| (0..5).all(move |row| *self.marks.get(&(row, col)).unwrap()))
    }

    fn score(&self) -> u32 {
        self.marks
            .iter()
            .filter(|(_, mark)| !**mark)
            .map(|(pos, _)| self.positions.get(pos).unwrap())
            .sum()
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = HashMap::default();
        let mut numbers = HashMap::default();

        s.lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(move |(col, n)| (n.parse::<u32>().unwrap(), (row, col)))
            })
            .for_each(|(n, (row, col))| {
                positions.insert((row, col), n);
                numbers.insert(n, (row, col));
            });

        let marks = (0..5)
            .flat_map(|row| (0..5).map(move |col| ((row, col), false)))
            .collect();

        Ok(Board {
            numbers,
            marks,
            positions,
        })
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                if *self.marks.get(&(row, col)).unwrap() {
                    write!(
                        f,
                        "\x1b[0;32m{:02?} \x1b[0m",
                        self.positions.get(&(row, col)).unwrap()
                    )
                    .unwrap();
                } else {
                    write!(f, "{:02?} ", self.positions.get(&(row, col)).unwrap()).unwrap();
                }
            }
            writeln!(f).unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let input = include_str!("test.input.txt");
        let (part1, part2) = solve(input);
        assert_eq!(part1, 4512);
        assert_eq!(part2, 1924);
    }
}
