use regex::Regex;

use utility_library::read_file;

fn part_1(input_lines: &String) -> (u32, u32) {
    let split_lines = input_lines.split("\n");
    let regex_pattern = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();


    let mut full_overlap_counter: u32 = 0;
    let mut partly_overlap_counter: u32 = 0;

    for line in split_lines {
        let captures = regex_pattern.captures(line).unwrap();

        let elf_1_start = &captures[1].parse::<i32>().unwrap();
        let elf_1_stop = &captures[2].parse::<i32>().unwrap();
        let elf_2_start = &captures[3].parse::<i32>().unwrap();
        let elf_2_stop = &captures[4].parse::<i32>().unwrap();

        if !(elf_1_stop - elf_2_start < 0 || elf_2_stop - elf_1_start < 0) {
            partly_overlap_counter += 1;

            if (elf_1_start >= elf_2_start && elf_1_stop <= elf_2_stop)
                || (elf_2_start >= elf_1_start && elf_2_stop <= elf_1_stop)
            {
                full_overlap_counter += 1;
            }
        }
    }

    let value_tuple = (full_overlap_counter, partly_overlap_counter);

    return value_tuple;
}

fn main() {
    let lines = read_file("/home/didrik/IdeaProjects/aoc/day_4/test_input.txt");

    let (part_1_result, part_2_result) = part_1(&lines);

    println!("Part-1: {part_1_result}");
    println!("Part-2: {part_2_result}");
}
