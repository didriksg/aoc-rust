use std::collections::HashSet;
use utility_library::read_file;

fn solver(input_signal: &str, value_needed: usize) -> usize {
    let mut start_pointer = 0;
    let mut end_pointer = value_needed;
    let str_length = input_signal.len();

    let mut combination_found = false;
    while !combination_found {
        let bab = input_signal[start_pointer..end_pointer].chars();
        let current_signal: HashSet<char> = HashSet::from_iter(bab);
        combination_found = current_signal.len() == value_needed;

        if end_pointer > str_length {
            break;
        }

        start_pointer += 1;
        end_pointer += 1;
    }
    return end_pointer - 1;
}


fn main() {
    let lines = read_file("/home/didrik/IdeaProjects/aoc/day_6/input.txt");

    for line in lines.split("\n") {
        let part_1_pos = solver(line, 4);
        let part_2_pos = solver(line, 14);

        println!("Part-1: {part_1_pos}");
        println!("Part-2: {part_2_pos}");
    }
}
