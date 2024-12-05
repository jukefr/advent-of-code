use regex::Regex;
use std::fs;

fn part_1(input_string: &str) -> u32 {
    let multiplication_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    input_string
        .lines()
        .flat_map(|line| multiplication_regex.captures_iter(line))
        .filter_map(|capture_groups| {
            let first_number: u32 = capture_groups[1].parse().ok()?;
            let second_number: u32 = capture_groups[2].parse().ok()?;
            Some(first_number * second_number)
        })
        .sum()
}

fn part_2(input_string: &str) -> u32 {
    let action_regex = Regex::new(
        r"(?x)
        mul\((\d+),(\d+)\) |
        do\(\) |
        don't\(\)
    ",
    )
    .unwrap();
    let mut is_action_enabled = true;
    input_string
        .lines()
        .flat_map(|line| action_regex.captures_iter(line))
        .filter_map(|capture_groups| match &capture_groups[0] {
            "do()" => {
                is_action_enabled = true;
                None
            }
            "don't()" => {
                is_action_enabled = false;
                None
            }
            _multiplication_action if is_action_enabled => {
                let first_number = capture_groups[1].parse::<u32>().ok()?;
                let second_number = capture_groups[2].parse::<u32>().ok()?;
                Some(first_number * second_number)
            }
            _ => None,
        })
        .sum()
}

fn main() {
    let input_file_contents = fs::read_to_string("input").expect("Failed to read input file");
    println!("[Part 1] {}", part_1(&input_file_contents));
    println!("[Part 2] {}", part_2(&input_file_contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input_string =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(test_input_string), 161);
    }
    #[test]
    fn test_part_2() {
        let test_input_string =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(test_input_string), 48);
    }
}
