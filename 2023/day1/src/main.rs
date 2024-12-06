use std::fs;

fn part_1(input: &str) -> u32 {
    let first_number: Vec<_> = input
        .lines()
        .map(|line| line.chars().find(|char| char.is_digit(10)))
        .collect();
    let last_number: Vec<_> = input
        .lines()
        .map(|line| line.chars().rev().find(|char| char.is_digit(10)))
        .collect();
    first_number
        .iter()
        .zip(last_number.iter())
        .filter_map(|(first, last)| {
            if let (Some(f), Some(l)) = (first, last) {
                Some((f, l))
            } else {
                None
            }
        })
        .map(|(f, l)| {
            let combined_digits = format!("{}{}", f, l);
            combined_digits.parse::<u32>().unwrap()
        })
        .sum()
}

fn main() {
    let input_data = fs::read_to_string("input").expect("Failed to read input file");
    println!("[Part 1] {}", part_1(&input_data));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input_string = "\
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";
        assert_eq!(part_1(test_input_string.trim()), 142);
    }
}
