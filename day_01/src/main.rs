fn main() {
    let input = include_str!("./input.txt");
    let numbers = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();

    let part1 = solve(&numbers, 1);
    assert_eq!(part1, 1557);
    println!("{} measurements increased", part1);

    let part2 = solve(&numbers, 3);
    assert_eq!(part2, 1608);
    println!("{} measurements increased", part2);
}

fn solve(input: &[usize], window_size: usize) -> usize {
    let windows = input
        .windows(window_size)
        .map(|w| w.iter().sum())
        .collect::<Vec<usize>>();

    windows
        .iter()
        .zip(windows.iter().skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}
