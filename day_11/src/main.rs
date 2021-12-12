use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    let cavern = Cavern::from_str(include_str!("input.txt")).unwrap();
    let (part1, part2) = solve(cavern);
    assert_eq!(part1, 1608);
    assert_eq!(part2, 214);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(mut cavern: Cavern) -> (usize, usize) {
    let mut flashes_after_100 = 0;
    let mut first_synchronized_flash = 0;

    for step in 1..10000 {
        let flashes = cavern.tick();
        if step == 100 {
            flashes_after_100 = cavern.total_flashes;
        }
        if flashes == cavern.inner.len() {
            first_synchronized_flash = step;
            break;
        }
    }
    (flashes_after_100, first_synchronized_flash)
}

struct Cavern {
    inner: HashMap<(i32, i32), u32>,
    total_flashes: usize,
}

impl Cavern {
    fn neighbours(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
        vec![
            (x + 1, y + 1),
            (x - 1, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y),
            (x - 1, y),
        ]
        .into_iter()
    }

    fn tick(&mut self) -> usize {
        let mut resets = HashSet::new();
        let mut flashing = VecDeque::new();
        let mut flashes = 0;

        self.inner.iter_mut().for_each(|(position, value)| {
            *value += 1;
            if *value > 9 {
                flashing.push_back(*position);
            }
        });

        while let Some((x, y)) = flashing.pop_front() {
            resets.insert((x, y));
            flashes += 1;
            for neighbour_position in Self::neighbours(x, y) {
                if let Some(neighbour) = self.inner.get_mut(&neighbour_position) {
                    *neighbour += 1;
                    if *neighbour == 10 {
                        flashing.push_back(neighbour_position);
                        resets.insert(neighbour_position);
                    }
                }
            }
        }

        // Reset all flashed ones at the end
        for r in &resets {
            if let Some(energy) = self.inner.get_mut(r) {
                *energy = 0;
            }
        }

        self.total_flashes += flashes;
        flashes
    }
}

impl FromStr for Cavern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(col, c)| ((row as i32, col as i32), c.to_digit(10).unwrap()))
            })
            .collect();

        Ok(Self {
            inner,
            total_flashes: 0,
        })
    }
}
