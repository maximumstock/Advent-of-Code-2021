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

        let mut annotation = Annotation::default();

        // Annotate 1
        for c in one.chars() {
            annotation.top_right.push(c);
            annotation.bottom_right.push(c);
        }

        // Annotate top
        let top_segment = left_str_minus_right_str(seven, one);
        annotation.top.extend(top_segment);

        // Annotate 4 -> top_left, middle
        let four_extra_segments = left_str_minus_right_str(four, seven);
        for c in four_extra_segments {
            annotation.top_left.push(c);
            annotation.middle.push(c);
        }

        // Annotate 8 -> bottom_left, bottom_middle
        let mut four_plus_seven = four.clone();
        four_plus_seven.push_str(seven);
        let eight_extra_segments = left_str_minus_right_str(eight, &four_plus_seven);
        for c in eight_extra_segments {
            annotation.bottom_left.push(c);
            annotation.bottom.push(c);
        }

        let permutations = annotation.permutations();

        let mut winner_mapping = HashMap::default();
        'outer: for p in permutations {
            let mut mapping = HashMap::new();
            for input in &self.inputs[3..=8] {
                if p.is_two(input) {
                    mapping.insert(input, 2);
                } else if p.is_three(input) {
                    mapping.insert(input, 3);
                } else if p.is_five(input) {
                    mapping.insert(input, 5);
                } else if p.is_six(input) {
                    mapping.insert(input, 6);
                } else if p.is_nine(input) {
                    mapping.insert(input, 9);
                } else if p.is_zero(input) {
                    mapping.insert(input, 0);
                }
            }

            if mapping.len() == 6 {
                mapping.insert(one, 1);
                mapping.insert(seven, 7);
                mapping.insert(four, 4);
                mapping.insert(eight, 8);

                winner_mapping = mapping;
                break 'outer;
            }
        }

        self.outputs
            .iter()
            .flat_map(|o| winner_mapping.get(o))
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
            .map(|s| sort_str(s))
            .collect::<Vec<String>>();
        let outputs = parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| sort_str(s))
            .collect::<Vec<String>>();

        Ok(Self { inputs, outputs })
    }
}

#[derive(Debug, Default)]
struct Annotation {
    top: Vec<char>,
    middle: Vec<char>,
    bottom: Vec<char>,
    top_left: Vec<char>,
    bottom_left: Vec<char>,
    top_right: Vec<char>,
    bottom_right: Vec<char>,
}

#[derive(Debug)]
struct Permutation {
    top: char,
    middle: char,
    bottom: char,
    top_left: char,
    bottom_left: char,
    top_right: char,
    bottom_right: char,
}

impl Permutation {
    fn is_two(&self, input: &str) -> bool {
        Self::check(
            &[
                self.top,
                self.top_right,
                self.middle,
                self.bottom_left,
                self.bottom,
            ],
            input,
        )
    }

    fn is_three(&self, input: &str) -> bool {
        Self::check(
            &[
                self.top,
                self.top_right,
                self.middle,
                self.bottom_right,
                self.bottom,
            ],
            input,
        )
    }

    fn is_five(&self, input: &str) -> bool {
        Self::check(
            &[
                self.top,
                self.top_left,
                self.middle,
                self.bottom_right,
                self.bottom,
            ],
            input,
        )
    }

    fn is_six(&self, input: &str) -> bool {
        Self::check(
            &[
                self.top,
                self.top_left,
                self.middle,
                self.bottom_right,
                self.bottom_left,
                self.bottom,
            ],
            input,
        )
    }

    fn is_nine(&self, input: &str) -> bool {
        Self::check(
            &[
                self.top,
                self.top_left,
                self.top_right,
                self.middle,
                self.bottom_right,
                self.bottom,
            ],
            input,
        )
    }

    fn is_zero(&self, input: &str) -> bool {
        Self::check(
            &[
                self.top,
                self.top_left,
                self.top_right,
                self.bottom_right,
                self.bottom_left,
                self.bottom,
            ],
            input,
        )
    }

    fn check(haystack: &[char], needle: &str) -> bool {
        needle.chars().all(|c| haystack.iter().any(|h| h.eq(&c)))
    }
}

impl Annotation {
    fn permutations(&self) -> Vec<Permutation> {
        let mut permutations = vec![];

        for top in &self.top {
            for top_left in &self.top_left {
                for top_right in &self.top_right {
                    for middle in &self.middle {
                        for bottom_left in &self.bottom_left {
                            for bottom_right in &self.bottom_right {
                                for bottom in &self.bottom {
                                    permutations.push(Permutation {
                                        top: *top,
                                        top_left: *top_left,
                                        top_right: *top_right,
                                        bottom: *bottom,
                                        bottom_left: *bottom_left,
                                        bottom_right: *bottom_right,
                                        middle: *middle,
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }

        permutations
    }
}

fn sort_str(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<_>>();
    chars.sort_unstable();
    chars.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Line;

    #[test]
    fn test_solve() {
        let mut line = Line::from_str(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();

        assert_eq!(line.solve(), 5353);
    }
}
