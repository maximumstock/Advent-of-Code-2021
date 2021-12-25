use std::collections::HashMap;

fn main() {
    let (p1, p2) = (7usize, 4usize);

    let part1 = solve_rec(p1, p2);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 675024);

    let part2 = solve_rec_2(p1, p2);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 570239341223618);
}

fn solve_rec_2(p1: usize, p2: usize) -> usize {
    let positions = [p1, p2];
    let scores = [0, 0];
    let mut dp = HashMap::new();

    let (p1, p2) = solve_rec_do_2(&mut dp, scores, positions, 0);
    p1.max(p2)
}

fn solve_rec_do_2(
    dp: &mut HashMap<([usize; 2], [usize; 2], usize), (usize, usize)>,
    scores: [usize; 2],
    positions: [usize; 2],
    roll: usize,
) -> (usize, usize) {
    if let Some(stored) = dp.get(&(scores, positions, roll)) {
        return *stored;
    }

    if scores[0] >= 21 {
        return (1, 0);
    }
    if scores[1] >= 21 {
        return (0, 1);
    }

    let mut outcomes = (0, 0);
    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                let mut scores = scores;
                let mut positions = positions;
                let steps = x + y + z;
                let pid = roll % 2;
                positions[pid] = (positions[pid] + steps) % 10;
                if positions[pid] == 0 {
                    positions[pid] = 10;
                }
                scores[pid] += positions[pid];

                let outcome = solve_rec_do_2(dp, scores, positions, roll + 1);
                dp.insert((scores, positions, roll + 1), outcome);

                outcomes = (outcomes.0 + outcome.0, outcomes.1 + outcome.1);
            }
        }
    }

    outcomes
}

fn solve_rec(p1: usize, p2: usize) -> usize {
    let die = &mut (1..=100usize).cycle();
    let positions = [p1, p2];
    let scores = [0, 0];

    let (score, _) = solve_rec_do(scores, positions, 1, die);

    score
}

fn solve_rec_do(
    mut scores: [usize; 2],
    mut positions: [usize; 2],
    roll: usize,
    die: &mut impl Iterator<Item = usize>,
) -> (usize, bool) {
    let steps = die.take(3).sum::<usize>();
    let pid = (roll - 1) % 2;
    positions[pid] = (positions[pid] + steps) % 10;
    if positions[pid] == 0 {
        positions[pid] = 10;
    }
    scores[pid] += positions[pid];

    if scores[0] > 1000 {
        return (scores[1] * 3 * roll, true);
    } else if scores[1] > 1000 {
        return (scores[0] * 3 * roll, false);
    }

    solve_rec_do(scores, positions, roll + 1, die)
}
