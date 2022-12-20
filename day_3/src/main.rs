fn char_to_score(current_char: char) -> u32 {
    let char_value = current_char as u32;

    let value = if current_char.is_uppercase() {
        char_value - 38
    } else {
        char_value - 96
    };

    return value;
}


fn main() {
    let input_string = include_str!("../input.txt");

    let part_1_result: u32 = input_string
        .split("\n")
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| a
            .chars()
            .filter(|a| b.contains(*a))
            .map(|a| char_to_score(a))
            .next()
            .unwrap()
        )
        .sum();

    let part_2_result: u32 = input_string
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|strings| strings[0]
            .chars()
            .filter(|c| strings[1].contains(*c) && strings[2].contains(*c))
            .map(|c| char_to_score(c))
            .next()
            .unwrap()
        )
        .sum();


    println!("Part 1: {part_1_result}");
    println!("Part 2: {part_2_result}");
}
