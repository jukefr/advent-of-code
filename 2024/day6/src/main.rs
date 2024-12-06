use std::fs;

fn print_map(map: &Vec<Vec<char>>) {
    map.iter()
        .map(|line| line.iter().collect::<String>())
        .for_each(|line| println!("{}", line));
    println!("");
}

fn part_1(input: &str) -> u32 {
    let mut map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut guard_position = None;
    let mut guard_direction = None;
    let directions = vec![
        (-1, 0), // ^
        (0, 1),  // >
        (1, 0),  // v
        (0, -1), // <
    ];
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' || cell == 'v' || cell == '<' || cell == '>' {
                guard_position = Some((i, j));
                guard_direction = Some(match cell {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => unreachable!(),
                });
                break;
            }
        }
    }
    let (mut x, mut y) = guard_position.unwrap();
    let mut direction = guard_direction.unwrap();
    map[x][y] = 'X';
    let mut out_of_bounds = false;
    while !out_of_bounds {
        let (dx, dy) = directions[direction];
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x < 0 || new_x >= map.len() as isize || new_y < 0 || new_y >= map[0].len() as isize {
            out_of_bounds = true;
            break;
        }
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        if map[new_x][new_y] == '#' {
            direction = (direction + 1) % 4;
        } else {
            x = new_x;
            y = new_y;
            if map[x][y] != 'X' {
                map[x][y] = 'X';
            }
        }
    }
    map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell == 'X')
        .count() as u32
}

fn part_2(input: &str) -> u32 {
    0
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
        let test_input_string = "\
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        ";
        assert_eq!(part_1(&test_input_string.trim()), 41);
    }
    #[test]
    fn test_part_2() {
        let test_input_string = "\
        ";
        assert_eq!(part_2(&test_input_string), 0);
    }
}
