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

#[allow(clippy::option_option)]
struct DoublePeekable<I: Iterator> {
    iter: std::iter::Peekable<I>,
    peeked: Option<Option<I::Item>>,
}

// based on std::iter::Peekable
// https://doc.rust-lang.org/src/core/iter/adapters/peekable.rs.html
#[allow(clippy::unused_peekable)]
impl<I: Iterator> DoublePeekable<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            peeked: None,
        }
    }

    fn peek_mut(&mut self) -> Option<&mut I::Item> {
        let iter = &mut self.iter;
        self.peeked.get_or_insert_with(|| iter.next()).as_mut()
    }

    fn peek2_mut(&mut self) -> Option<&mut I::Item> {
        self.peek_mut();
        self.iter.peek_mut()
    }
}

impl<I: Iterator> Iterator for DoublePeekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.iter.next(),
        }
    }
}

#[derive(Debug)]
struct Gear {
    index: i32,
    count: u32,
    ratio: u32,
}

impl Gear {
    const fn new(index: i32) -> Self {
        Self {
            index,
            count: 0,
            ratio: 1,
        }
    }

    const fn is_gear(&self) -> bool {
        self.count == 2
    }

    fn update(&mut self, number: u32) {
        if self.count >= 2 {
            self.count = 3;
            return;
        }
        self.count += 1;
        self.ratio *= number;
    }
}

#[allow(clippy::cognitive_complexity)]
pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut total = 0;
    let mut prev_numbers = DoublePeekable::new(vec![].into_iter());
    let mut prev_symbols = DoublePeekable::new(Vec::<Gear>::new().into_iter());
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
                b'*' => {
                    last_was_symbol = true;
                    let mut gear = Gear::new(i);
                    // Diagram:
                    //    s45e  <--- start/end of the numbers
                    // s41e     <-/
                    // .41.45   <-- previous line
                    // ...*
                    //    ^-- currently here
                    while let Some((s, e, num)) = prev_numbers.peek_mut() {
                        if i < *s {
                            break;
                        }
                        if i <= *e {
                            gear.update(*num);
                            if i == *e {
                                if let Some((s, _, num)) = prev_numbers.peek2_mut() {
                                    if i == *s {
                                        gear.update(*num);
                                    }
                                }
                                prev_numbers.next();
                            }
                            break;
                        }
                        prev_numbers.next();
                    }
                    curr_symbols.push(gear);
                }
                b'\n' => line_ending = true,
                _ => {}
            }
            if number > 0 || number_start > 0 {
                // don't use lws, we want if the current is symbol
                if last_was_symbol || insta_add_number {
                    if let Some(g) = curr_symbols.last_mut() {
                        g.update(number);
                    }
                }
                if last_was_symbol && insta_add_number {
                    let i = curr_symbols.len() - 2;
                    if let Some(g) = curr_symbols.get_mut(i) {
                        g.update(number);
                    }
                }
                let number_end = i;
                // Diagram:
                // *..*   <-- previous line
                // .41.45
                //    ^-- currently here
                while let Some(g) = prev_symbols.peek_mut() {
                    let s = g.index;
                    if s > number_end {
                        break;
                    }
                    if s >= number_start {
                        g.update(number);
                    }
                    if s == number_end {
                        if let Some(g) = prev_symbols.peek2_mut() {
                            if g.index == number_end {
                                g.update(number);
                            }
                        }
                        break;
                    }
                    if s < number_end {
                        if let Some(g) = prev_symbols.next() {
                            if g.is_gear() {
                                total += g.ratio;
                            }
                        }
                    }
                }
                curr_numbers.push((number_start, number_end, number));
                number = 0;
                number_start = 0;
                insta_add_number = false;
            }
        }
        if line_ending {
            for g in prev_symbols {
                if g.is_gear() {
                    total += g.ratio;
                }
            }
            prev_numbers = DoublePeekable::new(curr_numbers.into_iter());
            curr_numbers = vec![];
            prev_symbols = DoublePeekable::new(curr_symbols.into_iter());
            curr_symbols = vec![];
            index = 0;
        } else {
            index += 1;
        }
    }
    Some(total)
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
        assert_eq!(result, Some(467835));
    }
}
