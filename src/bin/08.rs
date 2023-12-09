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
}
