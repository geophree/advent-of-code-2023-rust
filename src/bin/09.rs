advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut input = input.as_bytes().iter();

    let mut total = 0;
    loop {
        let mut differences = vec![];
        let mut number = 0;
        let mut sign = 1;
        for c in input.by_ref() {
            if c.is_ascii_digit() {
                number *= 10;
                number += i32::from(*c - b'0');
                continue;
            }
            if *c == b'-' {
                sign = -1;
                continue;
            }
            number *= sign;
            let mut push_num = true;
            // TODO: just do this until first 0 0,
            // then we know how many end numbers we need, then work from end
            for n in &mut differences {
                if *n == 0 && number == 0 {
                    push_num = false;
                    break;
                }
                (*n, number) = (number, number - *n);
            }
            if push_num {
                differences.push(number);
            }
            if *c == b'\n' {
                break;
            }
            number = 0;
            sign = 1;
        }
        if let Some(next_term) = differences.into_iter().reduce(|m, n| m + n) {
            total += next_term;
        } else {
            break Some(total);
        }
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut nums: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let mut seq = vec![];
        for num in line.split(' ') {
            seq.push(num.parse().ok()?);
        }
        nums.push(seq);
    }
    let mut traps = vec![];
    for seq in nums {
        let mut trap = vec![];
        trap.push(seq);
        while trap.last()?.iter().any(|x| *x != 0) {
            trap.push(
                trap.last()?
                    .iter()
                    .scan(0, |p, n| {
                        let pp = *p;
                        *p = *n;
                        Some(*n - pp)
                    })
                    .skip(1)
                    .collect(),
            );
        }
        traps.push(trap);
    }
    let mut firsts_lasts: Vec<Vec<_>> = vec![];
    for seqs in &traps {
        let mut fls = vec![];
        for seq in seqs {
            fls.push((*(seq.first()?), *(seq.last()?)));
        }
        firsts_lasts.push(fls);
    }

    let mut prev_next = vec![];
    for fls in &firsts_lasts {
        let mut prev = 0;
        let mut next = 0;
        for (first, last) in fls.iter().rev() {
            prev = *first - prev;
            next += *last;
        }
        prev_next.push((prev, next));
    }
    let (total_prev, _total_next) = prev_next
        .iter()
        .copied()
        .reduce(|(tp, tn), (prev, next)| (tp + prev, tn + next))?;

    Some(total_prev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
