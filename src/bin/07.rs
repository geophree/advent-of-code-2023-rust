use itertools::Itertools;
use std::cmp::Ordering;
use std::marker::PhantomData;

advent_of_code::solution!(7);

trait Label:
    TryFrom<u8, Error = &'static str>
    + for<'a> TryFrom<&'a u8, Error = &'static str>
    + Into<usize>
    + PartialOrd
    + Ord
    + Copy
    + Clone
{
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum StandardLabel {
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

impl TryFrom<u8> for StandardLabel {
    type Error = &'static str;

    fn try_from(char: u8) -> Result<Self, Self::Error> {
        use StandardLabel::*;
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

impl TryFrom<&u8> for StandardLabel {
    type Error = &'static str;

    fn try_from(char: &u8) -> Result<Self, Self::Error> {
        (*char).try_into()
    }
}

impl From<StandardLabel> for usize {
    fn from(label: StandardLabel) -> Self {
        label as Self
    }
}

impl Label for StandardLabel {}

struct LabelCounts<L: Label> {
    counts: [u8; 13],
    phantom: PhantomData<L>,
}

impl<L: Label> LabelCounts<L> {
    fn new(counts: [u8; 13]) -> Self {
        Self {
            counts,
            phantom: Default::default(),
        }
    }
}

impl<L: Label> From<[u8; 13]> for LabelCounts<L> {
    fn from(counts: [u8; 13]) -> Self {
        Self::new(counts)
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

    fn upgrade_with_jokers(self, joker_count: u8) -> Result<Self, &'static str> {
        use Type::*;

        match (self, joker_count) {
            (t, 0) => Ok(t),
            (One, 1) => Ok(Pair),
            (One, 2) | (Pair, 1) => Ok(Three),
            (One, 3) | (Pair, 2) | (Three, 1) => Ok(Four),
            (One, 4) | (Pair, 3) | (Three, 2) | (Four, 1) => Ok(Five),
            (TwoPair, 1) => Ok(Full),
            _ => Err("too many cards"),
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

impl TryFrom<LabelCounts<StandardLabel>> for Type {
    type Error = &'static str;

    fn try_from(label_counts: LabelCounts<StandardLabel>) -> Result<Self, Self::Error> {
        label_counts
            .counts
            .into_iter()
            .filter_map(|v| v.try_into().ok())
            .try_fold(None::<Self>, |t, o| {
                t.map(|t| t.try_upgrade(o)).or(Some(Ok(o))).transpose()
            })
            .and_then(|t| t.ok_or("no cards!"))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand<T, L> {
    strength: (T, [L; 5]),
    bid: u32,
}

impl<T: TryFrom<LabelCounts<L>, Error = &'static str>, L: Label> Hand<T, L> {
    fn read<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Result<Self, &'static str> {
        let labels: [L; 5] = [
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
            input.next().ok_or("input ended early")?.try_into()?,
        ];
        let label_counts: LabelCounts<L> = labels
            .iter()
            .fold([0; 13], |mut counts, l| {
                counts[(*l).into()] += 1;
                counts
            })
            .into();
        input.next();
        let bid = read_u32(input).ok_or("failed to read bid")?;
        Ok(Self {
            strength: (label_counts.try_into()?, labels),
            bid,
        })
    }
}

impl<T: Ord, L: Ord> PartialOrd for Hand<T, L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord, L: Ord> Ord for Hand<T, L> {
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
        std::iter::from_fn(|| Hand::<Type, StandardLabel>::read(&mut input).ok())
            .sorted()
            .zip(1..)
            .map(|(h, i)| i * h.bid)
            .sum(),
    )
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum JokerLabel {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<u8> for JokerLabel {
    type Error = &'static str;

    fn try_from(char: u8) -> Result<Self, Self::Error> {
        use JokerLabel::*;
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
            b'J' => Ok(Joker),
            b'Q' => Ok(Queen),
            b'K' => Ok(King),
            b'A' => Ok(Ace),
            _ => Err("Unrecognized character"),
        }
    }
}

impl TryFrom<&u8> for JokerLabel {
    type Error = &'static str;

    fn try_from(char: &u8) -> Result<Self, Self::Error> {
        (*char).try_into()
    }
}

impl From<JokerLabel> for usize {
    fn from(label: JokerLabel) -> Self {
        label as Self
    }
}

impl Label for JokerLabel {}

impl TryFrom<LabelCounts<JokerLabel>> for Type {
    type Error = &'static str;

    fn try_from(label_counts: LabelCounts<JokerLabel>) -> Result<Self, Self::Error> {
        let mut counts = label_counts.counts;
        let i: usize = JokerLabel::Joker.into();
        let joker_count = counts[i];
        counts[i] = 0;
        let hand_type = counts
            .into_iter()
            .filter_map(|v| v.try_into().ok())
            .try_fold(None::<Self>, |t, o| {
                t.map(|t| t.try_upgrade(o)).or(Some(Ok(o))).transpose()
            })?;
        if let Some(hand_type) = hand_type {
            hand_type.upgrade_with_jokers(joker_count)
        } else {
            joker_count.try_into()
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.as_bytes().iter();
    Some(
        std::iter::from_fn(|| Hand::<Type, JokerLabel>::read(&mut input).ok())
            .sorted()
            .zip(1..)
            .map(|(h, i)| i * h.bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_hand() {
        use StandardLabel::Jack;
        let mut input = b"JJJJJ 10".iter();
        assert_eq!(
            Hand::<Type, StandardLabel>::read(&mut input),
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
        assert_eq!(result, Some(5905));
    }
}
