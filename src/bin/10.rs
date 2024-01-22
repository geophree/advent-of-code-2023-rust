advent_of_code::solution!(10);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn is_orthogonal(self, o: Self) -> bool {
        use Dir::*;
        matches!((self, o), (N | S, E | W) | (E | W, N | S))
    }
}

#[derive(Copy, Clone, Debug)]
enum SidedChange {
    NewlyIn,
    NewlyOut,
    NoChange,
}

#[derive(Copy, Clone, Debug)]
enum Wrap {
    Wrap(i16),
    OnLoop,
}

impl Default for Wrap {
    fn default() -> Self {
        Self::Wrap(0)
    }
}

impl Wrap {
    fn mark_on_loop(&mut self) -> SidedChange {
        let Self::Wrap(count) = *self else {
            return SidedChange::NoChange;
        };
        let ret = if count == 0 {
            SidedChange::NoChange
        } else {
            SidedChange::NewlyOut
        };
        *self = Self::OnLoop;
        ret
    }

    fn add_wrap(&mut self, dir: Dir) -> SidedChange {
        use Dir::*;
        let Self::Wrap(count) = *self else {
            return SidedChange::NoChange;
        };
        let change = match dir {
            N | E => 1,
            S | W => -1,
        };
        let (old_count, count) = (count, count + change);
        let ret = if old_count == 0 {
            SidedChange::NewlyIn
        } else if count == 0 {
            SidedChange::NewlyOut
        } else {
            SidedChange::NoChange
        };
        *self = Self::Wrap(count);
        ret
    }
}

struct WrapTracker {
    width: usize,
    // height: usize,
    wrap_counts: Vec<Wrap>,
    internal_count: u32,
}

impl WrapTracker {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            // height,
            wrap_counts: vec![Default::default(); width * height],
            internal_count: 0,
        }
    }

    fn index(&self, (row, col): (usize, usize)) -> usize {
        row * self.width + col
    }

    fn update_internal_count(&mut self, change: SidedChange) {
        match change {
            SidedChange::NewlyIn => self.internal_count += 1,
            SidedChange::NewlyOut => self.internal_count -= 1,
            SidedChange::NoChange => {}
        }
    }

    fn mark_on_loop(&mut self, pos: (usize, usize)) {
        let index = self.index(pos);
        let result = self.wrap_counts[index].mark_on_loop();
        self.update_internal_count(result);
    }

    fn add_wrap(&mut self, pos: (usize, usize), dir: Dir) {
        let index = self.index(pos);
        let result = self.wrap_counts[index].add_wrap(dir);
        self.update_internal_count(result);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut iter = input.iter();
    let pos1 = iter.position(|c| *c == b'\n' || *c == b'S').unwrap();
    let (width, start) = match input[pos1] {
        b'\n' => (pos1, pos1 + 1 + iter.position(|c| *c == b'S').unwrap()),
        b'S' => (pos1 + 1 + iter.position(|c| *c == b'\n').unwrap(), pos1),
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
/*
width = 5
 01234
0VVVVV
1>VVV<
2>>V<<
3>>^^<
4>^^^^

width = 6
 012345
0VVVVVV
1>VVVV<
2>>VV<<
3>>>^<<
4>>^^^<
5>^^^^^

 012345 col V: (col >= row && col + row + 1 <= width)
0012345     <: (col > row && col + row + 1 > width
1123456
2234567
3345678
4456789
5567890

row
>: (col < row && col + row + 1 <= width)
^: (col <= row && col + row + 1 > width)
*/

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let mut iter = input.iter();
    let pos1 = iter.position(|c| *c == b'\n' || *c == b'S').unwrap();
    let (width, start) = match input[pos1] {
        b'\n' => (pos1, pos1 + 1 + iter.position(|c| *c == b'S').unwrap()),
        b'S' => (pos1 + 1 + iter.position(|c| *c == b'\n').unwrap(), pos1),
        _ => return None,
    };
    let char_per_row = width + 1; // includes \n or EOF
    let start_row = start / char_per_row;
    let start_col = start % char_per_row;
    let height = input.len() / char_per_row;
    let index_rc = |row, col| row * char_per_row + col;
    let mut wrap_tracker = WrapTracker::new(width, height);
    wrap_tracker.mark_on_loop((start_row, start_col));
    let mut conns = vec![];
    if start_row > 0
        && matches!(
            input.get(index_rc(start_row - 1, start_col)),
            Some(b'|' | b'F' | b'7')
        )
    {
        conns.push(((start_row - 1, start_col), Dir::S));
    }
    if start_row < height - 1
        && matches!(
            input.get(index_rc(start_row + 1, start_col)),
            Some(b'|' | b'J' | b'L')
        )
    {
        conns.push(((start_row + 1, start_col), Dir::N));
    }
    if start_col > 0
        && matches!(
            input.get(index_rc(start_row, start_col - 1)),
            Some(b'-' | b'F' | b'L')
        )
    {
        conns.push(((start_row, start_col - 1), Dir::E));
    }
    if matches!(
        input.get(index_rc(start_row, start_col + 1)),
        Some(b'-' | b'J' | b'7')
    ) {
        conns.push(((start_row, start_col + 1), Dir::W));
    }
    // need to advance until we go perpendicular to our quadrant, that's the
    // spot we start after and travel back to.
    // save S next direction after we compute it to recall it when we get back
    let mut head = conns[1];
    let next_dir_from_start = head.1;
    let get_quadrant = |(row, col)| {
        if col + row < width {
            if col >= row {
                Dir::N
            } else {
                Dir::W
            }
        } else if col <= row {
            Dir::S
        } else {
            Dir::E
        }
        // if col >= row && col + row + 1 <= width {
        //     Dir::N
        // } else if col < row && col + row + 1 <= width {
        //     Dir::W
        // } else if col > row && col + row + 1 > width {
        //     Dir::E
        // } else if col <= row && col + row + 1 > width {
        //     Dir::S
        // }
    };

    let advance = |((row, col), dir)| {
        use Dir::*;
        let new_dir = match (input.get(index_rc(row, col)), dir) {
            (Some(b'|'), N) | (Some(b'F'), E) | (Some(b'7'), W) => N,
            (Some(b'|'), S) | (Some(b'J'), W) | (Some(b'L'), E) => S,
            (Some(b'-'), E) | (Some(b'7'), S) | (Some(b'J'), N) => E,
            (Some(b'-'), W) | (Some(b'F'), S) | (Some(b'L'), N) => W,
            (Some(b'S'), _) => next_dir_from_start,
            _ => panic!("Invalid input"),
        };
        match new_dir {
            N => ((row + 1, col), N),
            S => ((row - 1, col), S),
            E => ((row, col - 1), E),
            W => ((row, col + 1), W),
        }
    };

    let stop_after = loop {
        wrap_tracker.mark_on_loop(head.0);
        let quad = get_quadrant(head.0);
        let old_head = head;
        head = advance(head);
        if quad.is_orthogonal(head.1) {
            break old_head.0;
        }
    };

    let mut quad_line_entry_dir = None;
    loop {
        let done = head.0 == stop_after;
        wrap_tracker.mark_on_loop(head.0);
        let quad = get_quadrant(head.0);
        if quad.is_orthogonal(head.1) {
            quad_line_entry_dir = Some(head.1);
        }
        let new_head = advance(head);
        if quad.is_orthogonal(new_head.1) {
            if let Some(entry_dir) = quad_line_entry_dir {
                if entry_dir == new_head.1 {
                    let mut pos = head.0;
                    let advance = match quad {
                        Dir::N => |(row, col)| (row + 1, col),
                        Dir::S => |(row, col)| (row - 1, col),
                        Dir::E => |(row, col)| (row, col - 1),
                        Dir::W => |(row, col)| (row, col + 1),
                    };
                    loop {
                        pos = advance(pos);
                        if quad != get_quadrant(pos) {
                            break;
                        }
                        wrap_tracker.add_wrap(pos, entry_dir);
                    }
                }
            }
            quad_line_entry_dir = None;
        }
        if done {
            break;
        }
        let new_quad = get_quadrant(new_head.0);
        if quad != new_quad {
            quad_line_entry_dir = None;
        }
        head = new_head;
    }
    // println!("hello: {width} {height}");
    // for row in 0..height {
    //     for col in 0..width {
    //         let index = wrap_tracker.index((row, col));
    //         let s = match wrap_tracker.wrap_counts[index] {
    //             Wrap::OnLoop => "*".to_owned(),
    //             Wrap::Wrap(count) => {
    //                 let count = count.abs();
    //                 format!("{count}")
    //             },
    //         };
    //         print!("{s}");
    //     }
    //     println!("");
    // }
    Some(wrap_tracker.internal_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(23));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(22));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(70));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }
}
