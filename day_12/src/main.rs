use std::collections::{HashMap, VecDeque};

fn main() {
    let graph = parse_input(include_str!("input.txt"));
    let part1 = solve(&graph, 0);
    assert_eq!(part1, 5104);
    println!("Part 1: {}", part1);

    let part2 = solve(&graph, 1);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 149220);
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

#[derive(Clone, Default)]
struct Path<'a> {
    hops: Vec<&'a str>,
    small_caves_extra_visits: usize,
}

impl<'a> Path<'a> {
    fn is_reachable(&'a self, cave: &str) -> bool {
        self.is_repeatable_cave(cave)
            || !self.hops.contains(&cave)
            || cave.chars().next().unwrap().is_uppercase()
    }

    fn is_repeatable_cave(&self, cave: &str) -> bool {
        cave.chars().next().unwrap().is_lowercase()
            && cave.ne("start")
            && cave.ne("end")
            && self.small_caves_extra_visits > 0
            && self.hops.contains(&cave)
    }

    fn follow(&mut self, cave: &'a str) {
        if self.is_repeatable_cave(cave) {
            self.small_caves_extra_visits -= 1;
        }

        self.hops.push(cave);
    }
}

fn solve(graph: &Graph, small_caves_extra_visits: usize) -> usize {
    let mut correct_paths = 0;
    let mut queue = VecDeque::new();

    let mut p = Path {
        small_caves_extra_visits,
        ..Path::default()
    };
    p.follow("start");
    queue.push_back(p);

    while let Some(next_path) = queue.pop_front() {
        let from = next_path.hops.last().unwrap();
        if let Some(tos) = graph.get(from) {
            for to in tos {
                if to.eq(&"end") {
                    let mut p = next_path.clone();
                    p.follow("end");
                    correct_paths += 1;
                    continue;
                }

                if next_path.is_reachable(to) {
                    let mut new_p = next_path.clone();
                    new_p.follow(to);
                    queue.push_back(new_p);
                }
            }
        }
    }

    correct_paths
}

fn parse_input(input: &str) -> Graph {
    input
        .lines()
        .map(|l| {
            let mut parts = l.trim().split('-');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert_with(Vec::new).push(value);
            acc.entry(value).or_insert_with(Vec::new).push(key);
            acc
        })
}
