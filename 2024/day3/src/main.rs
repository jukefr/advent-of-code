use regex::Regex;
use std::fs;

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let re = Regex::new(
        r"(?x)
        mul\((\d+),(\d+)\)
        | do\(\)
        | don't\(\)
    ",
    )
    .unwrap();
    let mut sum = 0;
    let mut is_enabled = true;
    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            mul if is_enabled => {
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                sum += x * y;
            }
            _ => {}
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read the file");
    let result_part_1 = part_1(&input);
    println!("[Part 1] {}", result_part_1);
    let result_part_2 = part_2(&input);
    println!("[Part 2] {}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_result = 161;
        assert_eq!(part_1(input), expected_result);
    }
    #[test]
    fn test_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_result = 48;
        assert_eq!(part_2(input), expected_result);
    }
}
