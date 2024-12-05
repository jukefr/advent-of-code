use std::fs;

fn part_1(input: &str) -> u32 {
    let (mut left_values, mut right_values): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();
    left_values.sort_unstable();
    right_values.sort_unstable();
    left_values
        .iter()
        .zip(right_values.iter())
        .map(|(left_value, right_value)| left_value.abs_diff(*right_value))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let (left_values, right_values): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();
    let max_right_value = *right_values.iter().max().unwrap();
    let mut right_values_count = vec![0; max_right_value as usize + 1];
    right_values.iter().for_each(|&right_value| {
        right_values_count[right_value as usize] += 1;
    });
    left_values
        .iter()
        .map(|&left_value| {
            let count = right_values_count.get(left_value as usize).unwrap_or(&0);
            left_value * count
        })
        .sum()
}

fn main() {
    let input_data = fs::read_to_string("input").expect("Failed to read input file");
    println!("[Part 1] {}", part_1(&input_data));
    println!("[Part 2] {}", part_2(&input_data));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_STRING: &str = "\
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT_STRING.trim()), 11);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT_STRING.trim()), 31);
    }
}
