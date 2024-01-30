use std::collections::HashMap;

advent_of_code::solution!(8);

fn read_ident<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<u32> {
    match (input.next(), input.next(), input.next()) {
        (Some(a), Some(b), Some(c)) => Some(u32::from_ne_bytes([0, *a, *b, *c])),
        _ => None,
    }
}

fn read_node<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<(u32, (u32, u32))> {
    let key = read_ident(input)?;
    input.find(|c| **c == b'(');
    let left = read_ident(input)?;
    input.find(|c| **c == b' ');
    let right = read_ident(input)?;
    input.find(|c| **c == b'\n');
    Some((key, (left, right)))
}

const START: u32 = u32::from_ne_bytes([0, b'A', b'A', b'A']);
const END: u32 = u32::from_ne_bytes([0, b'Z', b'Z', b'Z']);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut iter = input.splitn(2, |&c| c == b'\n');
    let instructions = iter.next()?;
    let mut input = iter.next()?.iter();
    input.next();
    let map: HashMap<_, _> = std::iter::from_fn(|| read_node(&mut input)).collect();
    // println!("{instructions:?}");
    // println!("{map:?}");

    let mut steps = 0;
    let mut position = START;
    for turn in instructions.iter().cycle() {
        position = match (turn, map.get(&position)) {
            (b'L', Some((ident, _))) | (b'R', Some((_, ident))) => *ident,
            _ => None?,
        };
        steps += 1;
        if position == END {
            break;
        }
    }
    Some(steps)
}

// adapted from https://en.wikipedia.org/wiki/Binary_GCD_algorithm
// I don't think I need this?
// LCM(a, b) = a * (b / GCD(a, b))
pub fn gcd(mut u: u32, mut v: u32) -> u32 {
    match (u, v) {
        (0, x) | (x, 0) => return x,
        _ => {}
    }

    if u == v {
        return u;
    }

    let u_trailing = u.trailing_zeros();
    let v_trailing = v.trailing_zeros();
    let gcd_exponent_on_two = u_trailing.min(v_trailing);

    u >>= u_trailing;
    v >>= v_trailing;

    while u != v {
        if u < v {
            (u, v) = (v, u);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    u << gcd_exponent_on_two
}

const COUNTER_PRIMES: [u8; 8] = [5, 7, 11, 13, 17, 19, 23, 29];
const CHECK_STARTING_INDEX: usize = 2 + COUNTER_PRIMES.len();

// adapted from https://zsmith.co/primes.php
#[derive(Debug)]
struct PrimeIterator {
    primes: Vec<u32>,
    index: usize,
    counters: [u8; 8],
    check_to_index: usize,
    check_safe_until: u32,
    skip_four: bool,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            primes: vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97,
            ],
            index: 0,

            // these are based on starting at 97 (6*16 + 1)
            counters: [2, 6, 9, 6, 12, 2, 5, 10],
            // (we check (6*16 + 5) next
            skip_four: true,

            // this starts after the primes we keep counters for
            check_to_index: CHECK_STARTING_INDEX,
            check_safe_until: 841,
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index < self.primes.len() {
            self.index += 1;
            return Some(self.primes[index]);
        }

        let mut maybe_prime = self.primes[index - 1];
        loop {
            // all primes are (6n + 1) or (6n + 5), so alternate +2 and +4
            let inc: u8 = 2 + 2 * u8::from(self.skip_four);
            self.skip_four = !self.skip_four;

            maybe_prime += u32::from(inc);

            let mut is_composite = false;
            for (p, modulus) in COUNTER_PRIMES.iter().zip(self.counters.iter_mut()) {
                *modulus += inc;
                if *modulus >= *p {
                    *modulus -= p;
                    is_composite = is_composite || *modulus == 0;
                }
            }

            if is_composite {
                continue;
            }

            if maybe_prime > self.check_safe_until {
                self.check_to_index += 1;
                let highest_check_prime = self.primes[self.check_to_index];
                self.check_safe_until = highest_check_prime.saturating_mul(highest_check_prime);
            }

            for p in &self.primes[CHECK_STARTING_INDEX..=self.check_to_index] {
                is_composite = is_composite || maybe_prime % p == 0;
            }

            if is_composite {
                continue;
            }

            self.primes.push(maybe_prime);
            self.index += 1;
            break Some(maybe_prime);
        }
    }
}

#[derive(Debug)]
struct StepsToEndIter {
    steps_to_ends: Vec<u32>,
    loop_to_index: usize,
    i: usize,
}

impl StepsToEndIter {
    fn new(steps_to_ends: Vec<u32>, loop_to_index: usize) -> Self {
        Self {
            steps_to_ends,
            loop_to_index,
            i: 0,
        }
    }
}

impl Iterator for StepsToEndIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.steps_to_ends.len() - 1;
        if self.i > last {
            return None;
        }
        let i = self.i;
        if self.i == last {
            self.i = self.loop_to_index;
        } else {
            self.i += 1;
        }
        Some(self.steps_to_ends[i])
    }
}

const MASK: u32 = u32::from_ne_bytes([0, 0, 0, !0]);
const START_MASKED: u32 = u32::from_ne_bytes([0, 0, 0, b'A']);
const END_MASKED: u32 = u32::from_ne_bytes([0, 0, 0, b'Z']);
pub fn part_two(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let mut iter = input.splitn(2, |&c| c == b'\n');
    let instructions = iter.next()?;
    let mut input = iter.next()?.iter();
    input.next();
    let mut positions: Vec<_> = Default::default();
    let map: HashMap<_, _> = std::iter::from_fn(|| read_node(&mut input))
        .inspect(|(ident, _)| {
            if ident & MASK == START_MASKED {
                positions.push(*ident);
            }
        })
        .collect();

    let instruction_steps = instructions.len() as u32;
    let mut end_data: Vec<_> = positions
        .into_iter()
        .map(|mut position| {
            let mut steps = 0;
            let mut end_data = vec![];
            for turn in instructions.iter().cycle() {
                position = match (turn, map.get(&position)) {
                    (b'L', Some((ident, _))) | (b'R', Some((_, ident))) => *ident,
                    _ => panic!("position not found in map"),
                };
                steps += 1;
                if position & MASK == END_MASKED {
                    let mut cycle_steps = steps;
                    let mut loop_to_index = None;
                    for (i, (datum_steps, end_pos)) in end_data.iter().enumerate().rev() {
                        if *end_pos == position && cycle_steps % instruction_steps == 0 {
                            // we've found a full cycle.
                            loop_to_index = Some(i + 1);
                            break;
                        }
                        cycle_steps += *datum_steps;
                    }
                    end_data.push((steps, position));
                    if let Some(loop_to_index) = loop_to_index {
                        let steps_only: Vec<_> =
                            end_data.into_iter().map(|(steps, _)| steps).collect();
                        return StepsToEndIter::new(steps_only, loop_to_index)
                            .scan(0u64, |total, steps| {
                                *total += u64::from(steps);
                                Some(*total)
                            })
                            .peekable();
                    }
                    steps = 0;
                }
            }
            unreachable!();
        })
        .collect();
    loop {
        let mut all_equal = true;
        let highest = *end_data
            .iter_mut()
            .map(|it| it.peek().unwrap())
            .reduce(|highest, v| {
                all_equal = all_equal && v == highest;
                highest.max(v)
            })?;
        if all_equal {
            break Some(highest);
        }
        for it in &mut end_data {
            while *it.peek().unwrap() < highest {
                it.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(3, 6), 3);
        assert_eq!(gcd(4, 12), 4);
        assert_eq!(gcd(6, 12), 6);
        assert_eq!(gcd(6, 8), 2);

        // the following tests came from:
        // https://codereview.stackexchange.com/questions/183211/find-the-greatest-common-divisor-with-unit-tests
        assert_eq!(gcd(42, 56), 14);
        assert_eq!(gcd(461952, 116298), 18);
        assert_eq!(gcd(7966496, 314080416), 32);
        assert_eq!(gcd(24826148, 45296490), 526);
        assert_eq!(gcd(12, 0), 12);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 9), 9);
    }

    #[test]
    fn test_prime_iterator() {
        let mut it = PrimeIterator::new();
        let result = it.next();
        assert_eq!(result, Some(2));
        let result = it.next();
        assert_eq!(result, Some(3));
        let mut it = it.skip(22);
        let result = it.next();
        assert_eq!(result, Some(97));
        let result = it.next();
        assert_eq!(result, Some(101));
        let result = it.next();
        assert_eq!(result, Some(103));
        let mut it = it.skip(22);
        let result = it.next();
        assert_eq!(result, Some(229));
        let mut it = it.skip(1099);
        let result = it.next();
        assert_eq!(result, Some(9283));
        let mut it = it.skip(849);
        let result = it.next();
        assert_eq!(result, Some(17389));
    }
}
