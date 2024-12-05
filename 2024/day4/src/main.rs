use std::fs;

fn part_1(grid: &[Vec<char>]) -> usize {
    let directions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];
    let word = ['X', 'M', 'A', 'S'];
    let word_len = word.len();
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, _)| {
                let matches = directions
                    .iter()
                    .filter(|&&(dr, dc)| {
                        (0..word_len).all(|i| {
                            let nr = r as isize + dr * i as isize;
                            let nc = c as isize + dc * i as isize;
                            nr >= 0
                                && nr < grid.len() as isize
                                && nc >= 0
                                && nc < row.len() as isize
                                && grid[nr as usize][nc as usize] == word[i]
                        })
                    })
                    .count();
                if matches > 0 {
                    Some(matches)
                } else {
                    None
                }
            })
        })
        .sum::<usize>()
}

fn part_2(grid: &[Vec<char>]) -> usize {
    let patterns = [
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
    ];
    let expected_letters = [
        vec!['M', 'M', 'A', 'S', 'S'],
        vec!['S', 'S', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'S'],
        vec!['S', 'M', 'A', 'S', 'M'],
    ];
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map({
                let value = expected_letters.clone();
                move |(c, _)| {
                    let matches = patterns
                        .iter()
                        .zip(value.iter())
                        .filter(|(pattern, keys)| {
                            pattern.iter().enumerate().all(|(i, &(dr, dc))| {
                                let nr = r as isize + dr;
                                let nc = c as isize + dc;
                                nr >= 0
                                    && nr < grid.len() as isize
                                    && nc >= 0
                                    && nc < row.len() as isize
                                    && grid[nr as usize][nc as usize] == keys[i]
                            })
                        })
                        .count();
                    if matches > 0 {
                        Some(matches)
                    } else {
                        None
                    }
                }
            })
        })
        .sum::<usize>()
}

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    println!("[Part 1] {}", part_1(&grid));
    println!("[Part 2] {}", part_2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse_grid(input: &str) -> Vec<Vec<char>> {
        input
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect()
    }
    #[test]
    fn test_part_1() {
        let input = "\
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
        let grid = parse_grid(input);
        assert_eq!(part_1(&grid), 18);
    }
    #[test]
    fn test_part_2() {
        let input = "\
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
        let grid = parse_grid(input);
        assert_eq!(part_2(&grid), 9);
    }
}
