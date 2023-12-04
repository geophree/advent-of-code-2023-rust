advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut total = 0;
    let mut prev_numbers = vec![].into_iter().peekable();
    let mut prev_symbols = vec![].into_iter().peekable();
    let mut curr_numbers = vec![];
    let mut curr_symbols = vec![];
    let mut last_was_symbol = false;
    let mut number = 0;
    let mut number_start = 0;
    let mut insta_add_number = false;
    let mut index = 0;
    for &c in input {
        let lws = last_was_symbol;
        let i = index;
        let mut line_ending = false;
        last_was_symbol = false;
        if c.is_ascii_digit() {
            if number == 0 {
                number_start = i - 1;
                insta_add_number = lws;
            }
            number *= 10;
            number += u32::from(c - b'0');
        } else {
            match c {
                b'.' => {}
                b'\n' => line_ending = true,
                _ => {
                    last_was_symbol = true;
                    while let Some((s, e, num)) = prev_numbers.peek() {
                        if i < *s {
                            break;
                        }
                        if i <= *e {
                            total += *num;
                        }
                        prev_numbers.next();
                    }
                    curr_symbols.push(i);
                }
            }
            if number > 0 || number_start > 0 {
                // don't use lws, we want if the current is symbol
                if last_was_symbol || insta_add_number {
                    total += number;
                } else {
                    let number_end = i;
                    let mut number_added = false;
                    while let Some(&s) = prev_symbols.peek() {
                        if s > number_end {
                            break;
                        }
                        if s >= number_start && !number_added {
                            total += number;
                            number_added = true;
                        }
                        if s < number_end {
                            prev_symbols.next();
                        }
                        if number_added {
                            break;
                        }
                    }
                    if !number_added {
                        curr_numbers.push((number_start, number_end, number));
                    }
                }
                number = 0;
                number_start = 0;
                insta_add_number = false;
            }
        }
        if line_ending {
            prev_numbers = curr_numbers.into_iter().peekable();
            curr_numbers = vec![];
            prev_symbols = curr_symbols.into_iter().peekable();
            curr_symbols = vec![];
            index = 0;
        } else {
            index += 1;
        }
    }
    Some(total)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
