use std::collections::HashSet;
use utility_library::read_file;

fn solver(input_signal: &str, value_needed: usize) -> usize {
    let mut pointer = 0;
    let str_length = input_signal.len();

    loop {
        let end_point = pointer + value_needed;
        let current_signal: HashSet<char> = HashSet::from_iter(input_signal[pointer..end_point].chars());

        if current_signal.len() == value_needed || end_point >= str_length {
            break;
        }

        pointer += 1;
    }
    return pointer + value_needed;
}


fn main() {
    let lines = read_file("/home/didrik/git/aoc-rust/day_6/input.txt");

    for line in lines.split("\n") {
        let part_1_pos = solver(line, 4);
        let part_2_pos = solver(line, 14);

        println!("Part-1: {part_1_pos}");
        println!("Part-2: {part_2_pos}\n");
    }
}
