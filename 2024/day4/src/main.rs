use std::fs;

fn part_1(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word = ['X', 'M', 'A', 'S'];
    let word_len = word.len();
    let mut count = 0;
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
    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for i in 0..word_len {
                    let nr = r as isize + dr * i as isize;
                    let nc = c as isize + dc * i as isize;
                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != word[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_2(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    let in_bounds = |r: isize, c: isize| r >= 0 && r < rows as isize && c >= 0 && c < cols as isize;
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
    let matches_pattern = |r: usize, c: usize, pattern: &[(isize, isize)], keys: &[char]| -> bool {
        for (i, &(dr, dc)) in pattern.iter().enumerate() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if !in_bounds(nr, nc) {
                return false;
            }
            if let Some(&key) = keys.get(i) {
                if grid[nr as usize][nc as usize] != key {
                    return false;
                }
            }
        }
        true
    };
    for r in 0..rows {
        for c in 0..cols {
            for (pattern, keys) in patterns.iter().zip(expected_letters.iter()) {
                if matches_pattern(r, c, pattern, keys) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let count_part_1 = part_1(&grid);
    println!("[Part 1] {}", count_part_1);
    let count_part_2 = part_2(&grid);
    println!("[Part 2] {}", count_part_2);
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
        let grid = parse_grid(input.trim());
        let count = part_1(&grid);
        assert_eq!(count, 18);
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
        let grid = parse_grid(input.trim());
        let count = part_2(&grid);
        assert_eq!(count, 9);
    }
}
