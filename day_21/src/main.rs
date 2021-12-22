fn main() {
    let (p1, p2) = (7usize, 4usize);
    // let (p1, p2) = (4usize, 8usize);

    let part1 = solve(p1, p2);
    println!("Part 1: {}", part1);
    assert_eq!(part1, 675024);
}

fn solve(p1: usize, p2: usize) -> usize {
    let die = &mut (1..=100).cycle();
    let mut rolls = 0;
    let mut positions = [p1, p2];
    let mut scores = [0, 0];

    loop {
        for pid in 0..2 {
            let steps = die.take(3).sum::<usize>();
            rolls += 1;
            positions[pid] = (positions[pid] + steps) % 10;
            if positions[pid] == 0 {
                positions[pid] = 10;
            }
            scores[pid] += positions[pid];

            if scores[0].max(scores[1]) >= 1000 {
                return rolls * 3 * scores[0].min(scores[1]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
