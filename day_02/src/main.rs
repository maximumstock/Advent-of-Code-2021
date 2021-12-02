fn main() {
    let input = include_str!("./input.txt");
    let motions = input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let motion = parts.next().unwrap();
            let units = parts.next().unwrap().parse::<usize>().unwrap();

            match motion {
                "forward" => Motion::Forward(units),
                "up" => Motion::Up(units),
                "down" => Motion::Down(units),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Motion>>();

    let part1 = solve1(&motions);
    assert_eq!(part1, 1714680);
    println!("{} final position product", part1);

    let part2 = solve2(&motions);
    // assert_eq!(part2, 1608);
    println!("{} final position product", part2);
}

enum Motion {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn solve1(motions: &[Motion]) -> usize {
    // acc = (horizontal, depth)
    let (horizontal, depth) =
        motions
            .iter()
            .fold((0, 0), |(horizontal, depth), motion| match motion {
                Motion::Forward(units) => (horizontal + units, depth),
                Motion::Up(units) => (horizontal, depth - units),
                Motion::Down(units) => (horizontal, depth + units),
            });

    horizontal * depth
}

fn solve2(motions: &[Motion]) -> usize {
    // acc = (horizontal, depth, aim)
    let (horizontal, depth, _) =
        motions
            .iter()
            .fold((0, 0, 0), |(horizontal, depth, aim), motion| match motion {
                Motion::Forward(units) => (horizontal + units, depth + aim * units, aim),
                Motion::Up(units) => (horizontal, depth, aim - units),
                Motion::Down(units) => (horizontal, depth, aim + units),
            });

    horizontal * depth
}
