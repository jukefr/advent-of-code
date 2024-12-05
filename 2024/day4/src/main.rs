use std::fs;

fn part_1(grid: &[Vec<char>]) -> u32 {
    let direction_vectors = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];
    let target_word = ['X', 'M', 'A', 'S'];
    let target_word_length = target_word.len();
    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter().enumerate().filter_map(move |(col_index, _)| {
                let matching_directions_count = direction_vectors
                    .iter()
                    .filter(|&&(direction_row_offset, direction_col_offset)| {
                        (0..target_word_length).all(|word_index| {
                            let next_row_index =
                                row_index as isize + direction_row_offset * word_index as isize;
                            let next_col_index =
                                col_index as isize + direction_col_offset * word_index as isize;
                            next_row_index >= 0
                                && next_row_index < grid.len() as isize
                                && next_col_index >= 0
                                && next_col_index < row.len() as isize
                                && grid[next_row_index as usize][next_col_index as usize]
                                    == target_word[word_index]
                        })
                    })
                    .count();
                if matching_directions_count > 0 {
                    Some(matching_directions_count as u32)
                } else {
                    None
                }
            })
        })
        .sum::<u32>()
}

fn part_2(grid: &[Vec<char>]) -> u32 {
    let pattern_offsets = [
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
    ];
    let expected_characters = [
        vec!['M', 'M', 'A', 'S', 'S'],
        vec!['S', 'S', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'S'],
        vec!['S', 'M', 'A', 'S', 'M'],
    ];
    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter().enumerate().filter_map({
                let patterns_to_check = expected_characters.clone();
                move |(col_index, _)| {
                    let matching_patterns_count = pattern_offsets
                        .iter()
                        .zip(patterns_to_check.iter())
                        .filter(|(pattern_offset, expected_chars)| {
                            pattern_offset.iter().enumerate().all(
                                |(offset_index, &(row_offset, col_offset))| {
                                    let next_row_index = row_index as isize + row_offset;
                                    let next_col_index = col_index as isize + col_offset;
                                    next_row_index >= 0
                                        && next_row_index < grid.len() as isize
                                        && next_col_index >= 0
                                        && next_col_index < row.len() as isize
                                        && grid[next_row_index as usize][next_col_index as usize]
                                            == expected_chars[offset_index]
                                },
                            )
                        })
                        .count();
                    if matching_patterns_count > 0 {
                        Some(matching_patterns_count as u32)
                    } else {
                        None
                    }
                }
            })
        })
        .sum::<u32>()
}

fn main() {
    let input_file_contents = fs::read_to_string("input").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input_file_contents
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    println!("[Part 1] {}", part_1(&grid));
    println!("[Part 2] {}", part_2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse_grid_from_string(input_string: &str) -> Vec<Vec<char>> {
        input_string
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect()
    }
    #[test]
    fn test_part_1() {
        let test_input_string = "\
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        ";
        let grid = parse_grid_from_string(test_input_string);
        assert_eq!(part_1(&grid), 18);
    }
    #[test]
    fn test_part_2() {
        let test_input_string = "\
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
        ";
        let grid = parse_grid_from_string(test_input_string);
        assert_eq!(part_2(&grid), 9);
    }
}
