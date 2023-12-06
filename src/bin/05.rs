advent_of_code::solution!(5);

fn read_number<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<u32> {
    let mut number = 0;
    let mut saw_number = false;
    for c in input {
        if !c.is_ascii_digit() {
            break;
        }

        saw_number = true;
        number *= 10;
        number += u32::from(*c - b'0');
    }

    saw_number.then_some(number)
}

// keep list of slices, partition them based on new ranges
// once processed, add them to a (sorted?) list to combine later

// dest_start source_start range_length
pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.as_bytes().iter();

    let mut ids = vec![];

    input.find(|c| **c == b':');
    input.next();
    while let Some(seed) = read_number(&mut input) {
        ids.push(Some(seed));
    }

    // while input.find(|c| **c == b':').is_some() {
    while input.any(|&c| c == b':') {
        input.next();
        ids.sort();
        let mut first = 0;
        let mut last = ids.len() - 1;
        let mut next_ids = ids.clone();
        while let Some(dest_start) = read_number(&mut input) {
            let source_start = read_number(&mut input).unwrap();
            let range_length = read_number(&mut input).unwrap();
            let source_end = source_start + (range_length - 1);
            if ids[first] > Some(source_end) || ids[last] < Some(source_start) {
                continue;
            }
            let range = first..=last;
            for i in range {
                let mut o = ids[i];
                let Some(num) = o else {
                    first += usize::from(i == first);
                    continue;
                };
                if num < source_start {
                    continue;
                }
                if num > source_end {
                    break;
                }
                next_ids[i] = o.take().map(|num| num - source_start + dest_start);
                first += usize::from(i == first);
            }
            if first > last {
                break;
            }
            while ids[last].is_none() {
                last -= 1;
            }
        }
        ids = next_ids;
    }
    ids.into_iter().flatten().min()
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
