advent_of_code::solution!(10);

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut iter = input.iter();
    let pos1 = iter.position(|c| *c == b'\n' || *c == b'S').unwrap();
    let (width, start) = match input[pos1] {
        b'\n' => (pos1, pos1 + 1 + iter.position(|c| *c == b'S').unwrap()),
        // b'\n' => (pos1, input.iter().position(|c| *c == b'S').unwrap()),
        b'S' => (pos1 + 1 + iter.position(|c| *c == b'\n').unwrap(), pos1),
        // b'S' => (input.iter().position(|c| *c == b'\n').unwrap(), pos1),
        _ => return None,
    };
    let char_per_row = width + 1; // includes \n or EOF
    let start_row = start / char_per_row;
    let start_col = start % char_per_row;
    // let height = input.len() / char_per_row;
    let index_rc = |row, col| row * char_per_row + col;
    let mut conns = vec![];
    if start_row > 0
        && matches!(
            input.get(index_rc(start_row - 1, start_col)),
            Some(b'|' | b'F' | b'7')
        )
    {
        conns.push((start_row - 1, start_col, Dir::S));
    }
    if matches!(
        input.get(index_rc(start_row + 1, start_col)),
        Some(b'|' | b'J' | b'L')
    ) {
        conns.push((start_row + 1, start_col, Dir::N));
    }
    if start_col > 0
        && matches!(
            input.get(index_rc(start_row, start_col - 1)),
            Some(b'-' | b'F' | b'L')
        )
    {
        conns.push((start_row, start_col - 1, Dir::E));
    }
    if matches!(
        input.get(index_rc(start_row, start_col + 1)),
        Some(b'-' | b'J' | b'7')
    ) {
        conns.push((start_row, start_col + 1, Dir::W));
    }
    let mut head1 = conns[0];
    let mut head2 = conns[1];
    let advance = |(row, col, dir)| {
        use Dir::*;
        let new_dir = match (input.get(index_rc(row, col)), dir) {
            (Some(b'|'), N) | (Some(b'F'), E) | (Some(b'7'), W) => N,
            (Some(b'|'), S) | (Some(b'J'), W) | (Some(b'L'), E) => S,
            (Some(b'-'), E) | (Some(b'7'), S) | (Some(b'J'), N) => E,
            (Some(b'-'), W) | (Some(b'F'), S) | (Some(b'L'), N) => W,
            _ => panic!("Invalid input"),
        };
        match new_dir {
            N => (row + 1, col, N),
            S => (row - 1, col, S),
            E => (row, col - 1, E),
            W => (row, col + 1, W),
        }
    };
    let mut steps = 1;
    while (head1.0, head1.1) != (head2.0, head2.1) {
        head1 = advance(head1);
        head2 = advance(head2);
        steps += 1;
    }
    Some(steps)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
