use std::fs;

fn part_1(levels: &[u32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    levels.windows(2).all(|pair| {
        let level_difference = pair[1].abs_diff(pair[0]);
        if level_difference < 1 || level_difference > 3 {
            return false;
        }
        if pair[1] > pair[0] {
            is_decreasing = false;
        } else if pair[0] > pair[1] {
            is_increasing = false;
        }
        true
    }) && (is_increasing || is_decreasing)
}

fn part_2(levels: &[u32]) -> bool {
    if part_1(levels) {
        return true;
    }
    (0..levels.len()).any(|i| {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);
        part_1(&modified_levels)
    })
}

fn main() {
    let input_file_contents = fs::read_to_string("input").expect("Failed to read input file");
    let reports: Vec<Vec<u32>> = input_file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|number| number.parse().ok())
                .collect()
        })
        .collect();
    println!(
        "[Part 1] {}",
        reports.iter().filter(|&report| part_1(report)).count()
    );
    println!(
        "[Part 2] {}",
        reports.iter().filter(|&report| part_2(report)).count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_STRING: &str = "\
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";
    #[test]
    fn test_part_1() {
        let reports: Vec<Vec<u32>> = TEST_INPUT_STRING
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|number| number.parse().ok())
                    .collect()
            })
            .collect();
        assert_eq!(reports.iter().filter(|&report| part_1(report)).count(), 2);
    }
    #[test]
    fn test_part_2() {
        let reports: Vec<Vec<u32>> = TEST_INPUT_STRING
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|number| number.parse().ok())
                    .collect()
            })
            .collect();
        assert_eq!(reports.iter().filter(|&report| part_2(report)).count(), 4);
    }
}
