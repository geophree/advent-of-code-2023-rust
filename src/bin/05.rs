use range_set::range_set;

advent_of_code::solution!(5);

fn read_u32<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<u32> {
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
    while let Some(seed) = read_u32(&mut input) {
        ids.push(Some(seed));
    }

    // while input.find(|c| **c == b':').is_some() {
    while input.any(|&c| c == b':') {
        input.next();
        ids.sort();
        let mut first = 0;
        let mut last = ids.len() - 1;
        let mut next_ids = ids.clone();
        while let Some(dest_start) = read_u32(&mut input) {
            let source_start = read_u32(&mut input).unwrap();
            let range_length = read_u32(&mut input).unwrap();
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

fn read_u64<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<u64> {
    let mut number = 0;
    let mut saw_number = false;
    for c in input {
        if !c.is_ascii_digit() {
            break;
        }

        saw_number = true;
        number *= 10;
        number += u64::from(*c - b'0');
    }

    saw_number.then_some(number)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.as_bytes().iter();

    let mut id_ranges = range_set![];

    input.find(|c| **c == b':');
    input.next();
    // need read_u64 here because range_set doesn't play nice with ranges
    // containing T::MAX https://github.com/spearman/range-set/issues/14
    while let (Some(id_start), Some(length)) = (read_u64(&mut input), read_u64(&mut input)) {
        id_ranges.insert_range(id_start..=(id_start - 1 + length));
    }

    while input.any(|&c| c == b':') {
        input.next();
        let mut next_id_ranges = range_set![];
        while let Some(dest_start) = read_u64(&mut input) {
            let source_start = read_u64(&mut input).unwrap();
            let range_length = read_u64(&mut input).unwrap();
            let source_end = source_start + (range_length - 1);
            if let Some(rs) = id_ranges.remove_range(source_start..=source_end) {
                for r in rs.into_smallvec().into_iter() {
                    next_id_ranges.insert_range((r.start() - source_start + dest_start)..=(r.end() - source_start + dest_start));
                }
            }
        }
        for r in id_ranges.into_smallvec().into_iter() {
            next_id_ranges.insert_range(r);
        }
        id_ranges = next_id_ranges;
    }
    id_ranges.min().and_then(|x| u32::try_from(x).ok())
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
        assert_eq!(result, Some(46));
    }
}
