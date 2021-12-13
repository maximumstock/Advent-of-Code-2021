use std::{collections::HashSet, str::FromStr};

fn main() {
    let Paper {
        instructions,
        points,
    } = Paper::from_str(include_str!("input.txt")).unwrap();

    let part1 = solve(points.clone(), instructions.clone(), 1).len();
    assert_eq!(part1, 666);
    println!("Part 1: {}", part1);

    let part2 = solve(points, instructions, usize::MAX);
    render_points(&part2);
}

fn render_points(points: &HashSet<(u32, u32)>) {
    let (max_x, max_y) = points
        .iter()
        .fold((0, 0), |acc, next| (acc.0.max(next.0), acc.1.max(next.1)));

    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", points.get(&(x, y)).map(|_| "#").unwrap_or(" "));
        }
        println!();
    }
}

fn solve(
    points: HashSet<(u32, u32)>,
    instructions: Vec<(char, u32)>,
    folds: usize,
) -> HashSet<(u32, u32)> {
    instructions[0..folds.min(instructions.len())]
        .iter()
        .fold(points, |acc, (c, v)| fold(acc, *c, *v))
}

fn fold(mut points: HashSet<(u32, u32)>, axis: char, value: u32) -> HashSet<(u32, u32)> {
    let to_map = points
        .iter()
        .filter_map(|(x, y)| match axis {
            'y' if *y > value => Some(((*x, *y), (*x, value * 2 - *y))),
            'x' if *x > value => Some(((*x, *y), (value * 2 - *x, *y))),
            _ => None,
        })
        .collect::<Vec<_>>();

    for (old_point, new_point) in to_map {
        points.remove(&old_point);
        points.insert(new_point);
    }

    points
}

#[derive(Clone)]
struct Paper {
    points: HashSet<(u32, u32)>,
    instructions: Vec<(char, u32)>,
}

impl FromStr for Paper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let raw_points = parts.next().unwrap();
        let raw_instructions = parts.next().unwrap();

        let points = raw_points
            .lines()
            .map(|l| {
                let mut parts = l.trim().split(',').map(|x| x.parse().unwrap());
                (parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();

        let instructions = raw_instructions
            .lines()
            .map(|l| {
                let x = l.trim().split(' ').nth(2).unwrap();
                (x.chars().next().unwrap(), x[2..].parse().unwrap())
            })
            .collect();

        Ok(Paper {
            points,
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{render_points, solve, Paper};

    #[test]
    fn test_fold() {
        let paper = Paper::from_str(include_str!("test.input.txt")).unwrap();
        let points = solve(paper.points.clone(), paper.instructions.clone(), 1);
        assert_eq!(points.len(), 17);

        let part2 = solve(paper.points, paper.instructions, usize::MAX);
        render_points(&part2);
    }
}
