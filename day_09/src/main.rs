use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = parse_input(include_str!("input.txt"));

    let part1 = part1(&input);
    assert_eq!(part1, 436);
    println!("Part 1: {}", part1);

    let part2 = input.find_three_longest_basins_product();
    assert_eq!(part2, 1317792);
    println!("Part 2: {}", part2);
}

fn part1(height_map: &HeightMap) -> u32 {
    height_map
        .lowpoints()
        .iter()
        .map(|(_, height)| height + 1)
        .sum()
}

fn parse_input(input: &str) -> HeightMap {
    let inner = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((row, col), c.to_digit(10).unwrap()))
        })
        .collect::<HashMap<_, _>>();

    HeightMap { inner }
}

struct HeightMap {
    inner: HashMap<(usize, usize), u32>,
}

impl HeightMap {
    fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        vec![
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            Some((x + 1, y)),
            Some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
    }

    fn lowpoints(&self) -> Vec<((usize, usize), u32)> {
        self.inner
            .iter()
            .filter(|((x, y), height)| {
                Self::neighbours(*x, *y).all(|(xx, yy)| {
                    self.inner
                        .get(&(xx, yy))
                        .map(|v| *height < v)
                        .unwrap_or(true)
                })
            })
            .map(|(pos, height)| (*pos, *height))
            .collect::<Vec<_>>()
    }

    fn find_three_longest_basins_product(&self) -> usize {
        let mut basin_sizes = self
            .lowpoints()
            .iter()
            .map(|((x, y), _)| self.track_basin_size(*x, *y))
            .collect::<Vec<_>>();

        basin_sizes.sort_unstable();
        basin_sizes.iter().rev().take(3).product()
    }

    fn track_basin_size(&self, start_x: usize, start_y: usize) -> usize {
        let mut queue = VecDeque::<(usize, usize)>::default();
        let mut seen = HashSet::new();
        seen.insert((start_x, start_y));
        queue.push_back((start_x, start_y));

        while let Some(next) = queue.pop_front() {
            let height = self.inner.get(&next).unwrap();

            for neighbour in HeightMap::neighbours(next.0, next.1) {
                if seen.contains(&neighbour) {
                    continue;
                }

                if let Some(neighbour_height) = self.inner.get(&neighbour) {
                    if neighbour_height < &9 && neighbour_height > height {
                        queue.push_back(neighbour);
                        seen.insert(neighbour);
                    }
                }
            }
        }

        seen.len()
    }
}
