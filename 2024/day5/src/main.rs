use std::fs;

fn part_1(rule_strings: &Vec<&str>, update_strings: &Vec<&str>) -> u32 {
    let rule_pairs: Vec<Vec<u32>> = rule_strings
        .iter()
        .map(|rule_string| {
            rule_string
                .split('|')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    let update_pages_lists: Vec<Vec<u32>> = update_strings
        .iter()
        .map(|update_string| {
            update_string
                .split(',')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    update_pages_lists
        .iter()
        .map(|update_pages| {
            let is_valid_update = rule_pairs.iter().all(|rule_pair| {
                let (first_page, second_page) = (rule_pair[0], rule_pair[1]);
                let first_page_position = update_pages.iter().position(|&page| page == first_page);
                let second_page_position =
                    update_pages.iter().position(|&page| page == second_page);
                if let (Some(first_position), Some(second_position)) =
                    (first_page_position, second_page_position)
                {
                    first_position < second_position
                } else {
                    true
                }
            });
            if is_valid_update {
                update_pages[update_pages.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn part_2(rule_strings: &Vec<&str>, update_strings: &Vec<&str>) -> u32 {
    let rule_pairs: Vec<Vec<u32>> = rule_strings
        .iter()
        .map(|rule_string| {
            rule_string
                .split('|')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    let update_pages_lists: Vec<Vec<u32>> = update_strings
        .iter()
        .map(|update_string| {
            update_string
                .split(',')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    let invalid_update_pages_lists: Vec<Vec<u32>> = update_pages_lists
        .iter()
        .filter_map(|update_pages| {
            let is_valid_update = rule_pairs.iter().all(|rule_pair| {
                let (first_page, second_page) = (rule_pair[0], rule_pair[1]);
                let first_page_position = update_pages.iter().position(|&page| page == first_page);
                let second_page_position =
                    update_pages.iter().position(|&page| page == second_page);
                if let (Some(first_position), Some(second_position)) =
                    (first_page_position, second_page_position)
                {
                    first_position < second_position
                } else {
                    true
                }
            });
            if !is_valid_update {
                Some(update_pages.clone())
            } else {
                None
            }
        })
        .collect();
    let mut corrected_update_pages_lists = invalid_update_pages_lists.clone();
    let mut is_updates_corrected = false;
    while !is_updates_corrected {
        is_updates_corrected = true;
        let updated_invalid_update_pages_lists: Vec<Vec<u32>> = corrected_update_pages_lists
            .iter()
            .map(|invalid_update_pages| {
                rule_pairs
                    .iter()
                    .fold(invalid_update_pages.clone(), |mut update, rule_pair| {
                        let (first_page, second_page) = (rule_pair[0], rule_pair[1]);
                        let first_page_position =
                            update.iter().position(|&page| page == first_page);
                        let second_page_position =
                            update.iter().position(|&page| page == second_page);
                        if let (Some(first_position), Some(second_position)) =
                            (first_page_position, second_page_position)
                        {
                            if first_position > second_position {
                                update.swap(first_position, second_position);
                                is_updates_corrected = false;
                            }
                        }
                        update
                    })
            })
            .collect();
        corrected_update_pages_lists = updated_invalid_update_pages_lists;
    }
    corrected_update_pages_lists
        .iter()
        .map(|update_pages| update_pages[update_pages.len() / 2])
        .sum()
}

fn main() {
    let input_file_contents = fs::read_to_string("input").unwrap();
    let (rules_string, updates_string) = input_file_contents.split_once("\n\n").unwrap();
    let rule_strings: Vec<&str> = rules_string.lines().collect();
    let update_strings: Vec<&str> = updates_string.lines().collect();
    println!("[Part 1] {}", part_1(&rule_strings, &update_strings));
    println!("[Part 2] {}", part_2(&rule_strings, &update_strings));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_STRING: &str = "\
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";
    #[test]
    fn test_part_1() {
        let (rules_string, updates_string) = TEST_INPUT_STRING.trim().split_once("\n\n").unwrap();
        let rule_strings: Vec<&str> = rules_string.lines().collect();
        let update_strings: Vec<&str> = updates_string.lines().collect();
        assert_eq!(part_1(&rule_strings, &update_strings), 143);
    }
    #[test]
    fn test_part_2() {
        let (rules_string, updates_string) = TEST_INPUT_STRING.trim().split_once("\n\n").unwrap();
        let rule_strings: Vec<&str> = rules_string.lines().collect();
        let update_strings: Vec<&str> = updates_string.lines().collect();
        assert_eq!(part_2(&rule_strings, &update_strings), 123);
    }
}
