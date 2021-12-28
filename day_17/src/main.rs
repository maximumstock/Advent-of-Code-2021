use std::ops::RangeInclusive;

fn main() {
    let (target_x, target_y) = (57..=116, -198..=-148);

    let (max_y, n_velocities) = solve(target_x, target_y);
    println!("Part 1: {}", max_y);
    assert_eq!(max_y, 19503);

    println!("Part 2: {}", n_velocities);
    assert_eq!(n_velocities, 5200);
}

fn solve(target_x: RangeInclusive<i32>, target_y: RangeInclusive<i32>) -> (i32, i32) {
    (1..200)
        .flat_map(|x| (-200..200).map(move |y| (x, y)))
        .flat_map(|(x, y)| eval((x, y), target_x.clone(), target_y.clone()))
        .fold((0, 0), |acc, next| (acc.0.max(next), acc.1 + 1))
}

fn eval(
    mut velocity: (i32, i32),
    target_x: RangeInclusive<i32>,
    target_y: RangeInclusive<i32>,
) -> Option<i32> {
    let mut position = (0, 0);
    let mut max_y = 0;

    loop {
        position.0 += velocity.0;
        position.1 += velocity.1;
        max_y = max_y.max(position.1);
        velocity.0 = velocity.0 - velocity.0.signum();
        velocity.1 -= 1;

        if target_x.contains(&position.0) && target_y.contains(&position.1) {
            return Some(max_y);
        }

        if position.0 > *target_x.end() || position.1 < *target_y.start() {
            return None;
        }
    }
}
