use std::fs;

fn part_1(levels: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 {
            decreasing = false;
        } else if diff < 0 {
            increasing = false;
        }
    }
    increasing || decreasing
}

fn part_2(levels: &Vec<i32>) -> bool {
    if part_1(levels) {
        return true;
    }
    for i in 0..levels.len() {
        let mut new_levels = levels.clone();
        new_levels.remove(i);
        if part_1(&new_levels) {
            return true;
        }
    }
    false
}

fn analyze(input: &str, part: u8) -> usize {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let mut safe_count = 0;
    for report in reports {
        if part == 1 {
            if part_1(&report) {
                safe_count += 1;
            }
        } else if part == 2 {
            if part_2(&report) {
                safe_count += 1;
            }
        }
    }
    safe_count
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read the file");
    let safe_reports = analyze(&input, 1);
    println!("[Part 1] {}", safe_reports);
    let safe_reports = analyze(&input, 2);
    println!("[Part 2] {}", safe_reports);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse_input(input: &str) -> String {
        input.trim().to_string()
    }
    #[test]
    fn test_part_1() {
        let input = "\
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ";
        let parsed_input = parse_input(input);
        let total_score = analyze(&parsed_input, 1);
        assert_eq!(total_score, 2);
    }
    #[test]
    fn test_part_2() {
        let input = "\
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ";
        let parsed_input = parse_input(input);
        let total_score = analyze(&parsed_input, 2);
        assert_eq!(total_score, 4);
    }
}
