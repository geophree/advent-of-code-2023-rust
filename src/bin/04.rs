advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.as_bytes().iter();
    let mut total = 0;
    loop {
        let mut saw_colon = false;
        for &c in input.by_ref() {
            if c == b':' {
                saw_colon = true;
                break;
            }
        }

        if !saw_colon {
            break Some(total);
        }

        let mut winners = vec![];
        loop {
            input.next();
            match (input.next(), input.next()) {
                (Some(b'|'), _) => break,
                (Some(a), Some(b)) => {
                    winners.push(u32::from_ne_bytes([0, 0, *a, *b]));
                }
                _ => break,
            }
        }

        let mut matches = 0;
        loop {
            let (Some(a), Some(b)) = (input.next(), input.next()) else {
                break;
            };
            matches += u32::from(winners.contains(&u32::from_ne_bytes([0, 0, *a, *b])));
            if input.next() == Some(&b'\n') {
                break;
            }
        }

        if matches > 0 {
            total += 2u32.pow(matches - 1);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.as_bytes().iter();
    let mut total = 0;
    let mut card_counts = std::collections::VecDeque::new();
    loop {
        let mut saw_colon = false;
        for &c in input.by_ref() {
            if c == b':' {
                saw_colon = true;
                break;
            }
        }

        if !saw_colon {
            break Some(total);
        }

        let copies = card_counts.pop_front().unwrap_or(0) + 1;
        total += copies;

        let mut winners = vec![];
        loop {
            input.next();
            match (input.next(), input.next()) {
                (Some(b'|'), _) => break,
                (Some(a), Some(b)) => {
                    winners.push(u32::from_ne_bytes([0, 0, *a, *b]));
                }
                _ => break,
            }
        }

        let mut matches = 0;
        loop {
            let (Some(a), Some(b)) = (input.next(), input.next()) else {
                break;
            };
            matches += u32::from(winners.contains(&u32::from_ne_bytes([0, 0, *a, *b])));
            if input.next() == Some(&b'\n') {
                break;
            }
        }

        let mut adds = std::iter::repeat(copies).take(matches as usize);
        for (c, _) in card_counts.iter_mut().zip(adds.by_ref()) {
            *c += copies;
        }
        card_counts.extend(adds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
