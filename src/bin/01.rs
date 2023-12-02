advent_of_code::solution!(1);

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split('\n').fold(0, |total, s| {
        let tens = s
            .matches(|c| char::is_ascii_digit(&c))
            .next()
            .and_then(|c| c.parse().ok())
            .unwrap_or(0);
        let ones = s
            .rmatches(|c| char::is_ascii_digit(&c))
            .next()
            .and_then(|c| c.parse().ok())
            .unwrap_or(0);
        total + tens * 10 + ones
    }))
}

fn parse(input: &str) -> Option<u32> {
    match input {
        "0" | "zero" => Some(0),
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // as_bytes for speed?
    let re = regex!(r"[0-9]|one|two|three|four|five|six|seven|eight|nine");
    let rere = regex!(r"\A(?:[0-9]|one|two|three|four|five|six|seven|eight|nine)");
    Some(input.split('\n').fold(0, |total, s| {
        let mut found = false;
        let tens = re.find(s).map_or(0, |num| {
            found = true;
            parse(num.as_str()).unwrap_or(0)
        });
        let ones = if found {
            'ones: {
                for i in (0..s.len()).rev() {
                    if let Some(num) = rere.find(&s[i..]) {
                        break 'ones parse(num.as_str()).unwrap_or(0);
                    }
                }
                0
            }
        } else {
            0
        };
        total + tens * 10 + ones
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result1 = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result1, Some(142));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
