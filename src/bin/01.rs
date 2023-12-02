advent_of_code::solution!(1);

// macro_rules! regex {
//     ($re:literal $(,)?) => {{
//         static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
//         RE.get_or_init(|| regex::Regex::new($re).unwrap())
//     }};
// }

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut total = 0;
    let mut first = b'0';
    let mut last = b'0';
    for b in input {
        let b = *b;
        if b < b'1' {
            if b == b'\n' {
                total += ((first - b'0') * 10 + (last - b'0')) as u32;
                first = b'0';
                last = b'0';
            }
            continue;
        }

        if b > b'9' {
            continue;
        }

        if first == b'0' {
            first = b;
        }

        last = b;
    }
    if first != b'0' || last != b'0' {
        total += ((first - b'0') * 10 + (last - b'0')) as u32;
    }
    Some(total)
}

// fn parse(input: &str) -> Option<u32> {
//     match input {
//         "0" | "zero" => Some(0),
//         "1" | "one" => Some(1),
//         "2" | "two" => Some(2),
//         "3" | "three" => Some(3),
//         "4" | "four" => Some(4),
//         "5" | "five" => Some(5),
//         "6" | "six" => Some(6),
//         "7" | "seven" => Some(7),
//         "8" | "eight" => Some(8),
//         "9" | "nine" => Some(9),
//         _ => None,
//     }
// }

const WORDS: [(&[u8], u8); 9] = [
    (b"one", b'1'),
    (b"two", b'2'),
    (b"three", b'3'),
    (b"four", b'4'),
    (b"five", b'5'),
    (b"six", b'6'),
    (b"seven", b'7'),
    (b"eight", b'8'),
    (b"nine", b'9'),
];

// Using the FILTER actually slows down the code for some reason
// which letters appear in WORDS, found with:
// let mut letters: Vec<u8> = WORDS.iter().map(|(w,_)| w.iter().cloned()).flatten().collect();
// letters.sort_unstable();
// letters.dedup();
// let filter = letters.iter().fold(0u32, |acc, letter| {
//     acc | 1 << (letter - b'a')
// });
// println!("{}", std::str::from_utf8(&letters[..]).unwrap());
// println!("{filter:b}");
// const FILTER: u32 = 0b111111100110000111110000;

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut total = 0;
    let mut first = b'0';
    let mut last = b'0';
    let mut reset_words = false;
    let mut word_positions = [0;9];
    for b in input {
        let mut b = *b;
        if b < b'1' {
            if b == b'\n' {
                total += ((first - b'0') * 10 + (last - b'0')) as u32;
                first = b'0';
                last = b'0';
            }
            reset_words = true;
            continue;
        }

        if b > b'9' {
            if b < b'e' || b > b'x' { // || (FILTER >> (b - b'a')) & 1 == 0 {
                reset_words = true;
                continue;
            }
            if reset_words {
                word_positions = [0;9];
                reset_words = false;
            }
            let mut matched = false;
            let mut the_match = b'0';
            // this wouldn't work if the words had a second occurance of
            // their initial letter, "nine" would be a problem, but the
            // string that would trip this up "ninine" doesn't appear in the
            // input.
            for (pos, word_item) in word_positions.iter_mut().zip(WORDS) {
                let word = word_item.0;
                *pos = if b == word[*pos] {
                    let new = *pos + 1;
                    if new == word.len() {
                        matched = true;
                        the_match = word_item.1;
                        0
                    } else {
                        new
                    }
                } else if b == word[0] {
                    1
                } else {
                    0
                };
            }
            if matched {
                b = the_match;
            } else {
                continue;
            }
        }

        if first == b'0' {
            first = b;
        }

        last = b;
    }
    if first != b'0' || last != b'0' {
        total += ((first - b'0') * 10 + (last - b'0')) as u32;
    }
    Some(total)

    // this version is basically as fast, but much more straight-forward:
    //
    // let re = regex!(r"[0-9]|one|two|three|four|five|six|seven|eight|nine");
    // let rere = regex!(r"\A(?:[0-9]|one|two|three|four|five|six|seven|eight|nine)");
    // Some(input.split('\n').fold(0, |total, s| {
    //     let mut found = false;
    //     let tens = re.find(s).map_or(0, |num| {
    //         found = true;
    //         parse(num.as_str()).unwrap_or(0)
    //     });
    //     let ones = if found {
    //         'ones: {
    //             for i in (0..s.len()).rev() {
    //                 if let Some(num) = rere.find(&s[i..]) {
    //                     break 'ones parse(num.as_str()).unwrap_or(0);
    //                 }
    //             }
    //             0
    //         }
    //     } else {
    //         0
    //     };
    //     total + tens * 10 + ones
    // }))
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
