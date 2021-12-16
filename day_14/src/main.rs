use std::collections::HashMap;

fn main() {
    let (start, mapping) = parse_input(include_str!("input.txt"));

    let part1 = solve(start.clone(), &mapping, 10);
    assert_eq!(part1, 4244);
    println!("Part 1: {}", part1);

    let part2 = solve(start, &mapping, 40);
    assert_eq!(part2, 4807056953866);
    println!("Part 2: {}", part2);
}

fn solve(start: Vec<char>, mapping: &HashMap<(char, char), char>, iterations: usize) -> usize {
    let mut pair_counts =
        start
            .windows(2)
            .map(|w| (w[0], w[1]))
            .fold(HashMap::new(), |mut acc, next| {
                *acc.entry(next).or_insert(0) += 1;
                acc
            });

    let mut total_counts = HashMap::<char, usize>::new();

    for _ in 0..iterations {
        pair_counts =
            pair_counts
                .into_iter()
                .fold(HashMap::new(), |mut acc, ((left, right), count)| {
                    if let Some(m) = mapping.get(&(left, right)) {
                        *acc.entry((left, *m)).or_insert(0) += count;
                        *acc.entry((*m, right)).or_insert(0) += count;
                    }

                    acc
                });
    }

    for ((left, _), count) in pair_counts {
        *total_counts.entry(left).or_insert(0) += count;
    }
    *total_counts.entry(*start.last().unwrap()).or_default() += 1;

    let min = total_counts.iter().min_by_key(|e| e.1).unwrap().1;
    let max = total_counts.iter().max_by_key(|e| e.1).unwrap().1;
    max - min
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = input.lines();
    let start = lines.next().unwrap().chars().collect();
    let mapping = lines
        .skip(1)
        .map(|l| {
            let (left, right) = l.trim().split_once(" -> ").unwrap();
            let left = (left.chars().next().unwrap(), left.chars().nth(1).unwrap());
            (left, right.chars().next().unwrap())
        })
        .collect();

    (start, mapping)
}
