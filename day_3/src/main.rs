use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::Instant;

use utility_library::read_file;

fn char_to_score(current_char: char) -> u32 {
    let char_value = current_char as u32;

    let value = if current_char.is_uppercase() {
        char_value - 38
    } else {
        char_value - 96
    };

    return value;
}

fn check_priority_part_1(hash_set: HashSet<char>, line: &str) -> u32 {
    let mut priority = 0;
    for c in line.chars() {
        if hash_set.contains(&c) {
            priority = char_to_score(c);
            break;
        }
    }

    return priority;
}

fn part_1(input_lines: &String) -> u32 {
    let split_lines = input_lines.split("\n");
    let mut total_score = 0;

    for line in split_lines {
        let line_length = line.len();
        let half_line_length = line_length / 2;

        // Split lines at middle.
        let line_part_1 = &line[0..half_line_length];
        let line_part_2 = &line[half_line_length..line_length];

        // Create set of first part.
        let part_1_set: HashSet<char> = line_part_1.chars().collect();

        // Go through chars in other part and see if that char is in the hash map.
        total_score += check_priority_part_1(part_1_set, line_part_2);
    }

    return total_score;
}

fn part_2(input_lines: &String) -> u32 {
    let split_lines: Vec<&str> = input_lines.split("\n").collect();
    let total_lines = split_lines.len();

    let mut total_score = 0;
    for i in (0..total_lines).step_by(3) {
        let line_1 = split_lines[i + 0];
        let line_2 = split_lines[i + 1];
        let line_3 = split_lines[i + 2];

        let line_1_set: HashSet<char> = HashSet::from_iter(line_1.chars());
        let line_2_set: HashSet<char> = HashSet::from_iter(line_2.chars());

        for c in line_3.chars() {
            if line_1_set.contains(&c) && line_2_set.contains(&c) {
                total_score += char_to_score(c);
                break;
            }
        }
    }

    return total_score;
}


fn main() {
    let input_lines = read_file("/home/didrik/IdeaProjects/aoc/day_3/input.txt");

    let start = Instant::now();
    let part_1_result = part_1(&input_lines);
    let duration = start.elapsed();

    let start = Instant::now();
    let part_2_result = part_2(&input_lines);
    let duration_2 = start.elapsed();

    println!("Part 1: {}", part_1_result);
    println!("Part 2: {}", part_2_result);

    println!("Time part 1: {}", duration.as_micros());
    println!("Time part 2: {}", duration_2.as_micros());
}
