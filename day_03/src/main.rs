use std::iter::FromIterator;

fn main() {
    let numbers = parse_input(include_str!("./input.txt"));

    let (gamma, epsilon) = find_gamma_and_epsilon(&numbers, 12);
    let part1 = gamma as usize * epsilon as usize;
    assert_eq!(part1, 4006064);
    println!("power consumption: {}", part1);

    let part2 = solve2(&numbers, 12);
    assert_eq!(part2, 5941884);
    println!("life support rating: {}", part2);
}

fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|x| u16::from_str_radix(x.trim(), 2).unwrap())
        .collect::<Vec<_>>()
}

fn find_gamma_and_epsilon(numbers: &[u16], bits: u32) -> (usize, usize) {
    let mut gamma = (0..bits).fold(0, |mut g, offset| {
        let n_ones = count_ones_at_offset(numbers, bits - offset - 1);
        let n_zeroes = numbers.len() - n_ones;

        if n_ones > n_zeroes {
            g += 1;
        }

        g <<= 1;
        g
    });

    gamma >>= 1;

    let epsilon = 2_u16.pow(bits as u32) - 1 - gamma;

    (gamma as usize, epsilon as usize)
}

fn count_ones_at_offset(numbers: &[u16], digit: u32) -> usize {
    let mask = 1 << digit;
    numbers.iter().filter(|n| (*n & mask).eq(&mask)).count()
}

fn solve2(lines: &[u16], bits: u32) -> usize {
    let oxygen_generator_rating = find_rating(lines, true, bits);
    let co2_scrubber_rating = find_rating(lines, false, bits);

    oxygen_generator_rating * co2_scrubber_rating
}

fn find_rating(numbers: &[u16], most_common_bit: bool, bits: u32) -> usize {
    let mut numbers = Vec::from_iter(numbers.to_owned());

    for offset in (0..bits).rev() {
        let n_ones = count_ones_at_offset(&numbers, offset as u32);
        let n_zeroes = numbers.len() - n_ones;

        let expected_value = {
            if most_common_bit {
                if n_ones >= n_zeroes {
                    1
                } else {
                    0
                }
            } else if n_zeroes <= n_ones {
                0
            } else {
                1
            }
        };

        let mask = 1 << offset;
        numbers.retain(|n| ((n & mask) >> offset).eq(&expected_value));

        if numbers.len() == 1 {
            return *numbers.first().unwrap() as usize;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::{find_gamma_and_epsilon, parse_input, solve2};

    #[test]
    fn test_solve1() {
        let numbers = parse_input(include_str!("./test.input.txt"));
        let (gamma, epsilon) = find_gamma_and_epsilon(&numbers, 5);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn test_solve2() {
        let numbers = parse_input(include_str!("./test.input.txt"));
        assert_eq!(solve2(&numbers, 5), 230);
    }
}
