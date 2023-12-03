advent_of_code::solution!(2);

// macro_rules! regex {
//     ($re:literal $(,)?) => {{
//         static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
//         RE.get_or_init(|| regex::Regex::new($re).unwrap())
//     }};
// }

struct TupleMaker<'a, T> {
    input: &'a mut T,
    tuple: (u32, u32, u32),
    count: u32,
    last_was_space: bool,
    done: bool,
}

impl<'a, T> TupleMaker<'a, T>
where
    T: Iterator<Item = &'a u8>,
{
    fn new(input: &'a mut T) -> Self {
        Self {
            input,
            tuple: Default::default(),
            count: 0,
            last_was_space: false,
            done: false,
        }
    }

    fn next_line(&mut self) {
        if !self.done {
            for &c in &mut *self.input {
                if c == b'\n' {
                    break;
                }
            }
        }
        self.done = false;
    }

    fn consume_prefix(&mut self) -> bool {
        if !self.done {
            for &c in &mut *self.input {
                if c == b':' {
                    return true;
                }
            }
        }
        false
    }

    // fn consume_prefix_return_game_id(&mut self) -> Option<u32> {
    //     let mut game_id = 0;
    //     if self.done {
    //         return None;
    //     }
    //     for &c in &mut *self.input {
    //         match c {
    //             b'0'..=b'9' => {
    //                 game_id *= 10;
    //                 game_id += u32::from(c - b'0');
    //             },
    //             b':' => break,
    //             _ => {},
    //         }
    //     }
    //     if game_id != 0 {
    //         Some(game_id)
    //     } else {
    //         None
    //     }
    // }
}

impl<'a, T> Iterator for TupleMaker<'a, T>
where
    T: Iterator<Item = &'a u8>,
{
    type Item = (u32, u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        for &c in &mut *self.input {
            let last_was_space = self.last_was_space;
            self.last_was_space = c == b' ';
            match c {
                b'0'..=b'9' => {
                    self.count *= 10;
                    self.count += u32::from(c - b'0');
                }
                b'r' if last_was_space => {
                    self.tuple.0 = self.count;
                    self.count = 0;
                }
                b'g' if last_was_space => {
                    self.tuple.1 = self.count;
                    self.count = 0;
                }
                b'b' if last_was_space => {
                    self.tuple.2 = self.count;
                    self.count = 0;
                }
                b';' => return Some(std::mem::take(&mut self.tuple)),
                b'\n' => break,
                _ => {}
            }
        }
        self.done = true;
        Some(std::mem::take(&mut self.tuple))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let mine = (12, 13, 14);
    let mut input = input.as_bytes().iter();
    loop {
        let mut game_id = 0;
        for &c in input.by_ref() {
            match c {
                b'0'..=b'9' => {
                    game_id *= 10;
                    game_id += u32::from(c - b'0');
                }
                b':' => break,
                _ => {}
            }
        }
        if game_id == 0 {
            return Some(total);
        }

        let mut possible = true;
        let mut count = 0;
        let mut last_was_space = false;
        let mut done = false;
        for &c in input.by_ref() {
            let lws = last_was_space;
            last_was_space = c == b' ';
            match (c, mine) {
                (b'0'..=b'9', _) => {
                    count *= 10;
                    count += u32::from(c - b'0');
                }
                (b'r', (o, ..)) | (b'g', (_, o, ..)) | (b'b', (_, _, o)) if lws => {
                    possible = o >= count;
                    count = 0;
                    if !possible {
                        break;
                    }
                }
                (b'\n', _) => {
                    done = true;
                    break;
                }
                _ => {}
            }
        }

        if possible {
            total += game_id;
        }
        if !done {
            for &c in input.by_ref() {
                if c == b'\n' {
                    break;
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut input = input.as_bytes().iter();
    let mut tm = TupleMaker::new(&mut input);
    while tm.consume_prefix() {
        let mut mine = (0, 0, 0);
        for t in &mut tm {
            mine.0 = mine.0.max(t.0);
            mine.1 = mine.1.max(t.1);
            mine.2 = mine.2.max(t.2);
        }

        total += mine.0 * mine.1 * mine.2;
        tm.next_line();
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
