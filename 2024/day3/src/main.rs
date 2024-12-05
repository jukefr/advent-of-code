use regex::Regex;
use std::fs;

fn part_1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    input
        .lines()
        .flat_map(|line| regex.captures_iter(line))
        .filter_map(|capture| {
            let x: i32 = capture[1].parse().ok()?;
            let y: i32 = capture[2].parse().ok()?;
            Some(x * y)
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let regex = Regex::new(
        r"(?x)
        mul\((\d+),(\d+)\) |
        do\(\) |
        don't\(\)
    ",
    )
    .unwrap();
    let mut is_enabled = true;
    input
        .lines()
        .flat_map(|line| regex.captures_iter(line))
        .filter_map(|capture| match &capture[0] {
            "do()" => {
                is_enabled = true;
                None
            }
            "don't()" => {
                is_enabled = false;
                None
            }
            _mul if is_enabled => {
                let x = capture[1].parse::<i32>().ok()?;
                let y = capture[2].parse::<i32>().ok()?;
                Some(x * y)
            }
            _ => None,
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    println!("[Part 1] {}", part_1(&input));
    println!("[Part 2] {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(test_input), 161);
    }
    #[test]
    fn test_part_2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(test_input), 48);
    }
}
