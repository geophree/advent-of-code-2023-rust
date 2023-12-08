use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(7);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Label {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<u8> for Label {
    type Error = &'static str;

    fn try_from(char: u8) -> Result<Self, Self::Error> {
        use Label::*;
        match char {
            b'2' => Ok(Two),
            b'3' => Ok(Three),
            b'4' => Ok(Four),
            b'5' => Ok(Five),
            b'6' => Ok(Six),
            b'7' => Ok(Seven),
            b'8' => Ok(Eight),
            b'9' => Ok(Nine),
            b'T' => Ok(Ten),
            b'J' => Ok(Jack),
            b'Q' => Ok(Queen),
            b'K' => Ok(King),
            b'A' => Ok(Ace),
            _ => Err("Unrecognized character"),
        }
    }
}

impl TryFrom<&u8> for Label {
    type Error = &'static str;

    fn try_from(char: &u8) -> Result<Self, Self::Error> {
        (*char).try_into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    One,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl Type {
    fn try_upgrade(self, other: Self) -> Result<Self, &'static str> {
        use Type::*;

        match (self, other) {
            (Five, _) | (_, Five) => Err("Too many cards"),
            (a, One) | (One, a) => Ok(a),
            (Pair, Pair) => Ok(TwoPair),
            (Pair, Three) | (Three, Pair) => Ok(Full),
            _ => Err("Too many cards"),
        }
    }
}

impl TryFrom<u8> for Type {
    type Error = &'static str;

    fn try_from(count: u8) -> Result<Self, Self::Error> {
        use Type::*;

        match count {
            1 => Ok(One),
            2 => Ok(Pair),
            3 => Ok(Three),
            4 => Ok(Four),
            5 => Ok(Five),
            _ => Err("not a valid label count"),
        }
    }
}

impl TryFrom<[u8; 13]> for Type {
    type Error = &'static str;

    fn try_from(label_counts: [u8; 13]) -> Result<Self, Self::Error> {
        label_counts
            .into_iter()
            .filter_map(|v| v.try_into().ok())
            .try_fold(None::<Self>, |t, o| {
                t.map(|t| t.try_upgrade(o)).or(Some(Ok(o))).transpose()
            })
            .and_then(|t| t.ok_or("no cards!"))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    strength: (Type, [Label; 5]),
    bid: u32,
}

impl Hand {
    fn read<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Result<Self, &'static str> {
        let labels: [Label; 5] = [
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
        ];
        let label_counts = labels.iter().fold([0; 13], |mut counts, l| {
            counts[*l as usize] += 1;
            counts
        });
        input.next();
        let bid = read_u32(input).ok_or("failed to read bid")?;
        Ok(Self {
            strength: (label_counts.try_into()?, labels),
            bid,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength.cmp(&other.strength)
    }
}

fn read_u32<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<u32> {
    let mut number = 0;
    let mut saw_number = false;
    let mut skipping = true;
    for c in input {
        if skipping && *c == b' ' {
            continue;
        }
        skipping = false;
        if !c.is_ascii_digit() {
            break;
        }

        saw_number = true;
        number *= 10;
        number += u32::from(*c - b'0');
    }

    saw_number.then_some(number)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.as_bytes().iter();
    Some(
        std::iter::from_fn(|| Hand::read(&mut input).ok())
            .sorted()
            .zip(1..)
            .map(|(h, i)| i * h.bid)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_hand() {
        use Label::Jack;
        let mut input = b"JJJJJ 10".iter();
        assert_eq!(
            Hand::read(&mut input),
            Ok(Hand {
                strength: (Type::Five, [Jack, Jack, Jack, Jack, Jack]),
                bid: 10,
            })
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
