use std::str::FromStr;

advent_of_code::solution!(2);

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Debug, Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
    total: u32,
}

impl Cubes {
    const fn new(red: u32, green: u32, blue: u32) -> Self {
        Self {
            red,
            green,
            blue,
            total: red + green + blue,
        }
    }

    fn take_max(&mut self, other: &Self) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn is_possible_to_draw(&self, draw: &Self) -> bool {
        self.red >= draw.red
            && self.green >= draw.green
            && self.blue >= draw.blue
            && self.total >= draw.total
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCubesError;

impl FromStr for Cubes {
    type Err = ParseCubesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"([0-9]+) +(red|green|blue)");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for c in re.captures_iter(s) {
            let count = c[1].parse().or(Err(ParseCubesError))?;
            match &c[2] {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => Err(ParseCubesError)?,
            }
        }
        Ok(Self::new(red, green, blue))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let mine = Cubes::new(12, 13, 14);
    for s in input.split('\n') {
        let mut sections = s.split(':');
        let Some(game) = sections.next() else {
            continue;
        };
        let mut game = game.split(' ');
        game.next();
        let Some(game_id) = game.next().and_then(|x| x.parse::<u32>().ok()) else {
            continue;
        };
        let Some(rest) = sections.next() else {
            continue;
        };
        let possible = 'poss: {
            for d in rest.split(';') {
                let draw = &d.parse().ok()?;
                if !mine.is_possible_to_draw(draw) {
                    break 'poss false;
                }
            }
            true
        };
        if possible {
            total += game_id;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for s in input.split('\n') {
        let mut min_set = Cubes::default();
        let mut sections = s.split(':');
        sections.next();
        let Some(rest) = sections.next() else {
            continue;
        };
        for d in rest.split(';') {
            let draw = d.parse().ok()?;
            min_set.take_max(&draw);
        }
        total += min_set.power();
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
