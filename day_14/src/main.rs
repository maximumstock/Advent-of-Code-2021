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
    let mut pair_counts = start
        .windows(2)
        .map(|w| {
            let left = *w.first().unwrap();
            let right = *w.last().unwrap();
            (left, right)
        })
        .fold(HashMap::new(), |mut acc, next| {
            *acc.entry(next).or_insert(0) += 1;
            acc
        });

    let mut total_counts = HashMap::<char, usize>::new();
    *total_counts.entry(*start.last().unwrap()).or_default() += 1;

    for _ in 0..iterations {
        let mut update = HashMap::new();
        for ((left, right), v) in pair_counts {
            if let Some(m) = mapping.get(&(left, right)) {
                let new_pair = (left, *m);
                let new_pair2 = (*m, right);

                *update.entry(new_pair).or_insert(0) += v;
                *update.entry(new_pair2).or_insert(0) += v;
            }
        }
        pair_counts = update;
    }

    for ((left, _), count) in pair_counts {
        *total_counts.entry(left).or_insert(0) += count;
    }

    let min = total_counts.iter().min_by_key(|e| e.1).unwrap().1;
    let max = total_counts.iter().max_by_key(|e| e.1).unwrap().1;
    max - min
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut parts = input.split("\n\n");
    let start = parts.next().unwrap().chars().collect();
    let mapping = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.trim().split(" -> ");
            let mut left = parts.next().unwrap().chars();
            let left = (left.next().unwrap(), left.next().unwrap());
            (left, parts.next().unwrap().chars().next().unwrap())
        })
        .collect();

    (start, mapping)
}
