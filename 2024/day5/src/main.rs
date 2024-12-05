use std::fs;

fn part_1(rules: Vec<&str>, updates: Vec<&str>) -> u32 {
    let rules_numbers: Vec<Vec<u32>> = rules
        .iter()
        .map(|rule| {
            rule.split('|')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    let updates_numbers: Vec<Vec<u32>> = updates
        .iter()
        .map(|update| {
            update
                .split(',')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    updates_numbers
        .iter()
        .map(|update_pages| {
            let is_valid_update = rules_numbers.iter().all(|rule| {
                let (first_rule, second_rule) = (rule[0], rule[1]);
                let first_page_position = update_pages.iter().position(|&page| page == first_rule);
                let second_page_position =
                    update_pages.iter().position(|&page| page == second_rule);
                if let (Some(found_first_position), Some(found_second_position)) =
                    (first_page_position, second_page_position)
                {
                    found_first_position < found_second_position
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

fn part_2(rules: Vec<&str>, updates: Vec<&str>) -> u32 {
    let rules_numbers: Vec<Vec<u32>> = rules
        .iter()
        .map(|rule| {
            rule.split('|')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    let updates_numbers: Vec<Vec<u32>> = updates
        .iter()
        .map(|update| {
            update
                .split(',')
                .map(|page| page.trim())
                .filter_map(|page| page.parse::<u32>().ok())
                .collect()
        })
        .collect();
    let invalid_updates: Vec<Vec<u32>> = updates_numbers
        .iter()
        .filter_map(|update_pages| {
            let is_valid_update = rules_numbers.iter().all(|rule| {
                let (first_rule, second_rule) = (rule[0], rule[1]);
                let first_page_position = update_pages.iter().position(|&page| page == first_rule);
                let second_page_position =
                    update_pages.iter().position(|&page| page == second_rule);
                if let (Some(found_first_position), Some(found_second_position)) =
                    (first_page_position, second_page_position)
                {
                    found_first_position < found_second_position
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
    let mut fixed_updates = invalid_updates.clone();
    let mut updates_fixed = false;
    while !updates_fixed {
        updates_fixed = true;
        let updated_invalid_updates: Vec<Vec<u32>> = fixed_updates
            .iter()
            .map(|invalid_update| {
                rules_numbers
                    .iter()
                    .fold(invalid_update.clone(), |mut updated, rule| {
                        let (first, second) = (rule[0], rule[1]);
                        let first_pos = updated.iter().position(|&page| page == first);
                        let second_pos = updated.iter().position(|&page| page == second);
                        if let (Some(f_pos), Some(s_pos)) = (first_pos, second_pos) {
                            if f_pos > s_pos {
                                updated.swap(f_pos, s_pos);
                                updates_fixed = false;
                            }
                        }
                        updated
                    })
            })
            .collect();
        fixed_updates = updated_invalid_updates;
    }
    fixed_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let (rules_string, updates_string) = contents.split_once("\n\n").unwrap();
    let rules: Vec<&str> = rules_string.lines().collect();
    let updates: Vec<&str> = updates_string.lines().collect();
    println!("[Part 1] {}", part_1(rules.clone(), updates.clone()));
    println!("[Part 2] {}", part_2(rules.clone(), updates.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "\
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
        let (rules_string, updates_string) = test_input.trim().split_once("\n\n").unwrap();
        let rules: Vec<&str> = rules_string.lines().collect();
        let updates: Vec<&str> = updates_string.lines().collect();
        assert_eq!(part_1(rules, updates), 143);
    }
    #[test]
    fn test_part_2() {
        let test_input = "\
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
        let (rules_string, updates_string) = test_input.trim().split_once("\n\n").unwrap();
        let rules: Vec<&str> = rules_string.lines().collect();
        let updates: Vec<&str> = updates_string.lines().collect();
        assert_eq!(part_2(rules, updates), 123);
    }
}
