fn main() {
    let numbers = include_str!("input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let part1 = part1(&numbers);
    assert_eq!(part1, 352997);
    println!("Least Fuel - Part 1: {}", part1);

    let part2 = part2(&numbers);
    assert_eq!(part2, 101571302);
    println!("Least Fuel - Part 2: {}", part2);
}

fn part1(numbers: &[i32]) -> i32 {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let mut min_fuel = i32::MAX;

    for pos in min..=max {
        let cost = numbers.iter().map(|n| (n - pos).abs()).sum();
        min_fuel = min_fuel.min(cost);
    }

    min_fuel
}

fn part2(numbers: &[i32]) -> i32 {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    (min..=max).fold(i32::MAX, |acc, pos| {
        let cost = numbers
            .iter()
            .map(|n| (n - pos).abs())
            .map(|cost| cost * (cost + 1) / 2)
            .sum();
        acc.min(cost)
    })
}
