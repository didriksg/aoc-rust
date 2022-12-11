use utility_library::read_file;

fn sum_file(file_contents: String) -> Vec<i32> {
    let split_string = file_contents.split("\n");

    let mut summed_values: Vec<i32> = Vec::new();
    let mut current_line_value = 0;

    for s in split_string {
        if s.is_empty() {
            summed_values.push(current_line_value);
            current_line_value = 0;

            continue;
        }

        let line_value: i32 = s.parse().unwrap();
        current_line_value += line_value;
    }

    return summed_values;
}

fn summed_top_3(mut summed_values: Vec<i32>) -> i32 {
    summed_values.sort();

    let vector_size = summed_values.len();
    let top_3 = summed_values[vector_size-3..vector_size].iter().sum();

    return top_3;
}


pub fn main() {
    // Part 1:
    let file_path = "/home/didrik/IdeaProjects/aoc_testing/input_file.txt";
    let file_contents = read_file(file_path);
    let summed_values_vec = sum_file(file_contents);
    let largest_value = summed_values_vec.iter().max().unwrap();

    println!("Largest value: {}", largest_value);

    // Part 2:
    let summed_top_3 = summed_top_3(summed_values_vec);

    println!("Summed top-3: {}", summed_top_3);
}
