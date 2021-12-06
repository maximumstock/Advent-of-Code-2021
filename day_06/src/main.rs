fn main() {
    let input = include_str!("input.txt");
    let numbers = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let part1 = solve(&numbers, 80);
    println!("# Lanternfish after 80 days: {}", part1);

    let part2 = solve(&numbers, 256);
    println!("# Lanternfish after 256 days: {}", part2);
}

fn solve(numbers: &[usize], iterations: usize) -> usize {
    let mut arr = [0usize; 9];

    for n in numbers {
        arr[*n] += 1;
    }

    for _ in 0..iterations {
        for idx in 0..arr.len() - 1 {
            arr.swap(idx, idx + 1);
        }

        let oldest_fish = arr[8];
        arr[6] += oldest_fish;
    }

    arr.iter().sum()
}
