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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
