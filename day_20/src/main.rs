use std::{collections::HashMap, fmt::Write};

fn main() {
    let (program, map) = parse_input(include_str!("input.txt"));

    let part1 = solve(&program, map.clone(), 2);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 5619);

    let part2 = solve(&program, map, 50);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 20122);
}

fn solve(program: &[char], mut map: Map, iterations: usize) -> usize {
    for i in 0..iterations {
        map.tick(program, i);
    }
    map.lit()
}

fn parse_input(input: &str) -> (Vec<char>, Map) {
    let (raw_program, raw_map) = input.split_once("\n\n").unwrap();

    let inner = raw_map
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(col, c)| ((col as isize, row as isize), c.eq(&'#')))
        })
        .collect::<HashMap<_, _>>();

    let map = Map::new(inner);

    (raw_program.chars().collect(), map)
}

#[derive(Clone)]
struct Map {
    inner: HashMap<(isize, isize), bool>,
    dims: (isize, isize, isize, isize),
}

impl Map {
    fn new(inner: HashMap<(isize, isize), bool>) -> Self {
        Self {
            dims: Self::dims(&inner),
            inner,
        }
    }

    fn lit(&self) -> usize {
        self.inner.iter().filter(|(_, v)| **v).count()
    }

    fn fill(&mut self, iteration: usize) {
        // Everything around becomes false or true
        let d = 1;
        let v = iteration % 2 != 0;

        // top bottom
        for x in self.dims.0 - d..=self.dims.1 + d {
            self.inner.insert((x, self.dims.2 - 1), v);
            self.inner.insert((x, self.dims.3 + 1), v);
        }

        // left right
        for y in self.dims.2 - d..=self.dims.3 + d {
            self.inner.insert((self.dims.0 - 1, y), v);
            self.inner.insert((self.dims.1 + 1, y), v);
        }
        self.dims = Self::dims(&self.inner);
    }

    fn tick(&mut self, program: &[char], iteration: usize) {
        self.fill(iteration);
        let outside_default = if iteration % 2 == 0 { '0' } else { '1' };
        self.inner = self
            .inner
            .iter()
            .map(|(k, _)| {
                let convolution: usize = self.convolute(k, outside_default);
                let pixel = program[convolution];
                (*k, pixel == '#')
            })
            .collect();
    }

    fn convolute(&self, (x, y): &(isize, isize), outside_default: char) -> usize {
        let str = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .iter()
        .map(|(dx, dy)| match self.inner.get(&(x + dx, y + dy)) {
            Some(true) => '1',
            Some(false) => '0',
            None => outside_default,
        })
        .collect::<String>();

        usize::from_str_radix(&str, 2).unwrap()
    }

    fn dims(map: &HashMap<(isize, isize), bool>) -> (isize, isize, isize, isize) {
        map.keys().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |acc, &(x, y)| (acc.0.min(x), acc.1.max(x), acc.2.min(y), acc.3.max(y)),
        )
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f).unwrap();
        for y in self.dims.2..=self.dims.3 {
            for x in self.dims.0..=self.dims.1 {
                let ch = if *self.inner.get(&(x, y)).unwrap() {
                    '#'
                } else {
                    '.'
                };
                f.write_char(ch).unwrap();
            }
            writeln!(f).unwrap();
        }
        writeln!(f).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve};

    #[test]
    fn test_convolution() {
        let (_, mut map) = parse_input(include_str!("test.input.txt"));
        let convolution = map.convolute(&(2, 2), '0');
        assert_eq!(convolution, 34);

        map.fill(0);
        let convolution = map.convolute(&(3, 1), '0');
        assert_eq!(
            convolution,
            usize::from_str_radix("010000001", 2).unwrap() // 129
        );

        let convolution = map.convolute(&(4, 1), '0');
        assert_eq!(
            convolution,
            usize::from_str_radix("100000010", 2).unwrap() // 258
        );
    }

    #[test]
    fn test_part1() {
        let (program, map) = parse_input(include_str!("test.input.txt"));
        let part1 = solve(&program, map, 2);
        assert_eq!(part1, 35);
    }
}
