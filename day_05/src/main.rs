use std::{collections::HashMap, error::Error, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let lines = parse_input(input);

    let part1 = solve(&lines, &|line: &Line| {
        line.is_horizontal() || line.is_vertical()
    });
    println!("Part 1: {}", part1);

    let part2 = solve(&lines, &|_| true);
    println!("Part 2: {}", part2);
}

fn solve(lines: &[Line], selector: &dyn Fn(&Line) -> bool) -> usize {
    let point_count = lines
        .iter()
        .filter(|l| selector(l))
        .flat_map(|l| l.points())
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        });

    point_count.iter().filter(|(_, count)| **count > 1).count()
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect()
}

struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_vertical(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn points(&self) -> Vec<(u32, u32)> {
        if self.is_horizontal() {
            (self.start.1..=self.end.1)
                .map(|y| (self.start.0, y))
                .collect()
        } else if self.is_vertical() {
            (self.start.0..=self.end.0)
                .map(|x| (x, self.start.1))
                .collect()
        } else {
            let range_x: Vec<u32> = {
                if self.start.0 < self.end.0 {
                    (self.start.0..=self.end.0).collect()
                } else {
                    (self.end.0..=self.start.0).rev().collect()
                }
            };

            let range_y: Vec<u32> = {
                if self.start.1 < self.end.1 {
                    (self.start.1..=self.end.1).collect()
                } else {
                    (self.end.1..=self.start.1).rev().collect()
                }
            };

            (range_x.into_iter()).zip(range_y.into_iter()).collect()
        }
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let mut start = parts
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap());
        let mut end = parts
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap());

        let start = (start.next().unwrap(), start.next().unwrap());
        let end = (end.next().unwrap(), end.next().unwrap());

        let (start, end) = sort_tuples(start, end);

        Ok(Self { start, end })
    }
}

fn sort_tuples(t1: (u32, u32), t2: (u32, u32)) -> ((u32, u32), (u32, u32)) {
    let d1 = t1.0.pow(2) + t1.1.pow(2);
    let d2 = t2.0.pow(2) + t2.1.pow(2);

    if d2 >= d1 {
        (t1, t2)
    } else {
        (t2, t1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve, Line};

    #[test]
    fn test() {
        let input = include_str!("test.input.txt");
        let lines = parse_input(input);
        let part1 = solve(&lines, &|line| line.is_horizontal() || line.is_vertical());
        assert_eq!(part1, 5);
        let part2 = solve(&lines, &|_| true);
        assert_eq!(part2, 12);
    }

    #[test]
    fn test_points() {
        let line = Line {
            start: (5, 5),
            end: (8, 2),
        };
        assert_eq!(line.points(), vec![(5, 5), (6, 4), (7, 3), (8, 2)]);
    }
}
