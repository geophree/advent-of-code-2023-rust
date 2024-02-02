advent_of_code::solution!(11);

fn make_expansion_record(should_expand: Vec<bool>) -> Vec<(usize, usize)> {
    let mut expansion_record = vec![];
    let should_length = should_expand.len();
    let mut iter = should_expand.into_iter();
    let mut expansion = 0usize;
    let mut total_position = 0;
    while let Some(pos) = iter.position(|x| x) {
        if pos > 0 {
            // there were some occupied spots
            expansion_record.push((total_position, expansion));
        }
        total_position += pos + 1;
        expansion += 1;
    }
    if total_position < should_length {
        expansion_record.push((total_position, expansion));
    }
    expansion_record.push((should_length, expansion));
    expansion_record
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let iter = input.iter();
    let mut col_should_expand = vec![];
    let mut row_should_expand = vec![true];
    let mut galaxies = vec![];
    let mut row = 0;
    let mut col = 0;
    let mut width = None;
    for c in iter {
        if width.is_none() {
            col_should_expand.push(true);
        }
        match c {
            b'\n' => {
                if width.is_none() {
                    width = Some(col);
                    col_should_expand.pop();
                }
                row_should_expand.push(true);
                row += 1;
                col = 0;
                continue;
            }
            b'#' => {
                galaxies.push((col, row));
                row_should_expand[row] = false;
                col_should_expand[col] = false;
            }
            _ => {}
        }
        col += 1;
    }
    let Some(_width) = width else {
        return None;
    };
    row_should_expand.pop();
    let col_expansion = make_expansion_record(col_should_expand);
    let row_expansion = make_expansion_record(row_should_expand);
    let mut col_ei = 0;
    let mut row_ei = 0;

    for g in &mut galaxies {
        let (col, row) = *g;
        if col_expansion[col_ei].0 > col {
            col_ei = 0;
        }

        while col >= col_expansion[col_ei].0 {
            col_ei += 1;
        }
        col_ei -= 1;

        while row >= row_expansion[row_ei].0 {
            row_ei += 1;
        }
        row_ei -= 1;
        *g = (col + col_expansion[col_ei].1, row + row_expansion[row_ei].1);
    }

    Some(
        galaxies
            .iter()
            .enumerate()
            .map(|(pos, (col1, row1))| {
                galaxies
                    .iter()
                    .take(pos)
                    .map(|(col2, row2)| col1.abs_diff(*col2) + row1.abs_diff(*row2))
                    .sum::<usize>()
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let iter = input.iter();
    let mut col_should_expand = vec![];
    let mut row_should_expand = vec![true];
    let mut galaxies = vec![];
    let mut row = 0;
    let mut col = 0;
    let mut width = None;
    for c in iter {
        if width.is_none() {
            col_should_expand.push(true);
        }
        match c {
            b'\n' => {
                if width.is_none() {
                    width = Some(col);
                    col_should_expand.pop();
                }
                row_should_expand.push(true);
                row += 1;
                col = 0;
                continue;
            }
            b'#' => {
                galaxies.push((col, row));
                row_should_expand[row] = false;
                col_should_expand[col] = false;
            }
            _ => {}
        }
        col += 1;
    }
    let Some(_width) = width else {
        return None;
    };
    row_should_expand.pop();
    let col_expansion = make_expansion_record(col_should_expand);
    let row_expansion = make_expansion_record(row_should_expand);
    let mut col_ei = 0;
    let mut row_ei = 0;

    let expansion_factor = 1_000_000;

    let galaxies: Vec<_> = galaxies.into_iter().map(|(col, row)| {
        if col_expansion[col_ei].0 > col {
            col_ei = 0;
        }

        while col >= col_expansion[col_ei].0 {
            col_ei += 1;
        }
        col_ei -= 1;

        while row >= row_expansion[row_ei].0 {
            row_ei += 1;
        }
        row_ei -= 1;
        ((col + col_expansion[col_ei].1 * (expansion_factor - 1)) as u64, (row + row_expansion[row_ei].1 * (expansion_factor - 1)) as u64)
    }).collect();

    Some(
        galaxies
            .iter()
            .enumerate()
            .map(|(pos, (col1, row1))| {
                galaxies
                    .iter()
                    .take(pos)
                    .map(|(col2, row2)| col1.abs_diff(*col2) + row1.abs_diff(*row2))
                    .sum::<u64>()
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
