use regex::Regex;


fn parse_board(board: &str) -> Vec<Vec<&str>> {
    // Split "board" strings into rows.
    let board_vector: Vec<&str> = board.split("\n").collect();

    // All but last row is information about the stacks.
    let board_values = board_vector[0..board_vector.len() - 1].to_owned();
    let number_of_stacks = (board_vector[0].len() + 1) / 4;

    // Initiate stacks.
    let mut stacks: Vec<Vec<&str>> = Vec::new();
    for _ in 0..number_of_stacks {
        stacks.push(Vec::new());
    }

    // Insert row values into its associated stack.
    let regex_pattern: Regex = Regex::new(r"(\w+\s*)").unwrap();
    for board_value in board_values {
        for value in regex_pattern.find_iter(board_value) {
            // 1-index of position can be divided by 4 to get the correct stack.
            let value_pos = (value.start() + 1) / 4;
            let value_val = value.as_str();

            stacks[value_pos].insert(0, value_val);
        }
    }

    return stacks;
}

/// Moves and reverses order.
fn move_push_pop(board: &mut Vec<Vec<&str>>, move_count: usize, move_from_index: usize, move_to_index: usize) {
    // You achieve the same by reversing the order of the string slice in move_collect.
    for _ in 0..move_count {
        let move_val = board[move_from_index].pop().unwrap();
        board[move_to_index].push(move_val);
    }
}

/// Moves and keeps order.
fn move_collect(board: &mut Vec<Vec<&str>>, move_count: usize, move_from_index: usize, move_to_index: usize) {
    let drain_amount = board[move_from_index].len() - move_count;
    let mut move_values: Vec<&str> = board[move_from_index].drain(drain_amount..).collect();

    board[move_to_index].append(&mut move_values);
}

/// Collects the top elements of all stacks into a string.
fn generate_last_item_string(board: Vec<Vec<&str>>) -> String {
    let mut value_string: String = String::new();
    for stack in board {
        value_string.push_str(stack.last().unwrap());
    }

    return value_string;
}

/// Main function to process the instruction strings.
fn process_instructions_stack(board: Vec<Vec<&str>>, instruction_set: &str) {
    let instruction_pattern: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // Keep two separate boards for the two different parts.
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


fn main() {
    let lines = include_str!("../input.txt");
    let split_values: Vec<&str> = lines.split("\n\n").collect();

    let board = parse_board(split_values[0]);
    let info_lines = split_values[1];

    process_instructions_stack(board, info_lines);
}
