use utility_library::read_file;

use regex::Regex;


fn parse_board(board: &str) -> Vec<Vec<&str>> {
    let board_vector: Vec<&str> = board.split("\n").collect();
    let board_size = board_vector.len();
    let board_values = &board_vector[0..board_size - 1];
    let board_size = (board_vector[0].len() + 1) / 4;

    let mut stacks: Vec<Vec<&str>> = Vec::new();
    for _ in 0..board_size {
        stacks.push(Vec::new());
    }

    let regex_pattern: Regex = Regex::new(r"(\w+\s*)").unwrap();

    for bab in board_values {
        let values = regex_pattern.find_iter(bab);
        for value in values {
            let value_pos = value.start() / 4;
            let value_val = value.as_str();

            stacks[value_pos].insert(0, value_val);
        }
    }

    return stacks;
}

fn move_push_pop(board: &mut Vec<Vec<&str>>, move_count: usize, move_from_index: usize, move_to_index: usize) {
    for _ in 0..move_count {
        let move_val = board[move_from_index].pop().unwrap();
        board[move_to_index].push(move_val);
    }
}

fn move_collect(board: &mut Vec<Vec<&str>>, move_count: usize, move_from_index: usize, move_to_index: usize) {
    let drain_amount = board[move_from_index].len() - move_count;
    let mut move_values: Vec<&str> = board[move_from_index].drain(drain_amount..).collect();
    board[move_to_index].append(&mut move_values);
}

fn generate_last_item_string(board: Vec<Vec<&str>>) -> String {
    let mut value_string: String = "".parse().unwrap();
    for stack in &*board {
        value_string.push_str(stack.last().unwrap());
    }

    return value_string;
}


fn process_instructions_stack(board: &Vec<Vec<&str>>, instruction_set: &str) {
    let instruction_pattern: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut board_push_pop = board.clone();
    let mut board_collect = board.clone();

    for instruction in instruction_set.split("\n") {
        let instruction_captures = instruction_pattern.captures(instruction).unwrap();

        let move_count = instruction_captures[1].parse::<usize>().unwrap();
        let move_from_index = instruction_captures[2].parse::<usize>().unwrap() - 1;
        let move_to_index = instruction_captures[3].parse::<usize>().unwrap() - 1;

        move_push_pop(&mut board_push_pop, move_count, move_from_index, move_to_index);
        move_collect(&mut board_collect, move_count, move_from_index, move_to_index);
    }

    let last_item_string_part_1 = generate_last_item_string(board_push_pop);
    let last_item_string_part_2 = generate_last_item_string(board_collect);

    println!("Part-1: {last_item_string_part_1}");
    println!("Part-2: {last_item_string_part_2}");
}


fn part_1(input_lines: &String) {
    let split_values: Vec<&str> = input_lines.split("\n\n").collect();

    let board = parse_board(split_values[0]);
    let info_lines = split_values[1];

    process_instructions_stack(&board, info_lines);
}


fn main() {
    let lines = read_file("/home/didrik/IdeaProjects/aoc/day_5/test_input.txt");
    part_1(&lines);
}
