advent_of_code::solution!(6);

fn read_f64<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<f64> {
    let mut number = 0.0;
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
        number *= 10.0;
        number += f64::from(*c - b'0');
    }

    saw_number.then_some(number)
}

#[allow(clippy::cast_sign_loss)]
pub fn part_one(input: &str) -> Option<u32> {
    // x(time - x) = best
    // -x^2 + (time)x - best = 0
    // x^2 - (time)x + best = 0
    // x^2 -7x + 9 = 0
    // x = (-time +/- sqrt(time^2 - 4 * distance)) / 2

    let mut input = input.as_bytes().iter();
    input.find(|c| **c == b':');
    let mut times = vec![];
    while let Some(time) = read_f64(&mut input) {
        times.push(time);
    }

    input.find(|c| **c == b':');
    let mut distances = vec![];
    while let Some(dist) = read_f64(&mut input) {
        distances.push(dist);
    }
    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(time, dist)| {
                let add = 4f64.mul_add(-dist, time.powi(2)).sqrt();
                let high = (((time + add) / 2.0) - 1.0).ceil() as u32;
                let low = (((time - add) / 2.0) + 1.0).floor() as u32;
                high - low + 1
            })
            .product(),
    )
}

fn read_f64_ignore_spaces<'a, I: Iterator<Item = &'a u8>>(input: &mut I) -> Option<f64> {
    let mut number = 0.0;
    let mut saw_number = false;
    for c in input {
        if *c == b' ' {
            continue;
        }
        if !c.is_ascii_digit() {
            break;
        }

        saw_number = true;
        number *= 10.0;
        number += f64::from(*c - b'0');
    }

    saw_number.then_some(number)
}

#[allow(clippy::cast_sign_loss)]
pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.as_bytes().iter();
    input.find(|c| **c == b':');
    let Some(time) = read_f64_ignore_spaces(&mut input) else {
        return None;
    };

    input.find(|c| **c == b':');
    let Some(dist) = read_f64_ignore_spaces(&mut input) else {
        return None;
    };

    let add = 4f64.mul_add(-dist, time.powi(2)).sqrt();
    let high = (((time + add) / 2.0) - 1.0).ceil() as u32;
    let low = (((time - add) / 2.0) + 1.0).floor() as u32;
    Some(high - low + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
