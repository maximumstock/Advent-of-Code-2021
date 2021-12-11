use std::collections::VecDeque;

fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<&str>>();
    let part1 = part1(&lines);
    assert_eq!(part1, 344193);
    println!("Part 1: {}", part1);

    let part2 = part2(&lines);
    assert_eq!(part2, 3241238967);
    println!("Part 2: {}", part2);
}

fn part1(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| match score(line) {
            Ok(_) => 0,
            Err(error_score) => error_score,
        })
        .sum()
}

fn part2(lines: &[&str]) -> usize {
    let mut autocomplete_scores = lines
        .iter()
        .filter_map(|line| match score(line) {
            Ok(stack) => Some(stack),
            Err(_) => None,
        })
        .map(|stack| {
            stack.iter().rev().fold(0usize, |acc, next| {
                let rank = match next {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                acc * 5 + rank
            })
        })
        .collect::<Vec<_>>();

    autocomplete_scores.sort_unstable();

    let index = autocomplete_scores.len() / 2;
    *autocomplete_scores.get(index).unwrap()
}

fn score(line: &str) -> Result<VecDeque<char>, usize> {
    let mut stack = VecDeque::new();

    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => {
                stack.push_back(char);
            }
            ')' | ']' | '}' | '>' => {
                let error_score = expect_next(&mut stack, char);
                if error_score > 0 {
                    return Err(error_score);
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(stack)
}

fn expect_next(stack: &mut VecDeque<char>, expected: char) -> usize {
    match (expected, stack.pop_back()) {
        (')', Some('(')) => 0,
        (']', Some('[')) => 0,
        ('}', Some('{')) => 0,
        ('>', Some('<')) => 0,
        (_, Some(next)) if expected.ne(&next) => rank(expected),
        _ => unreachable!(),
    }
}

fn rank(char: char) -> usize {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}
