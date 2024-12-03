use std::collections::HashMap;
use std::fs;

fn part_1(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part_2(input: &str) -> i32 {
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();
    let mut right_counts = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }
    left.iter()
        .map(|&num| {
            let count = right_counts.get(&num).unwrap_or(&0);
            num * count
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let total_distance = part_1(&input);
    println!("[Part 1] {}", total_distance);
    let similarity_score = part_2(&input);
    println!("[Part 2] {}", similarity_score);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = "\
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        assert_eq!(part_1(input), 11);
    }
    #[test]
    fn test_part_2() {
        let input = "\
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        assert_eq!(part_2(input), 31);
    }
}
