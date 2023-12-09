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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
