fn is_between(number: i32, start: i32, end: i32, modulo: i32) -> bool {
    return start % modulo <= number % modulo && number % modulo <= end % modulo;
}

fn write_pixel(signal_vector: &mut Vec<&str>, current_cycle: &mut usize, register_value: i32) {
    signal_vector[*current_cycle] = if is_between(
        *current_cycle as i32,
        register_value - 1,
        register_value + 1,
        40
    ) { "#" } else { "." };
    *current_cycle += 1;
}

fn execute_commands(commands: &Vec<&str>) -> (i32, Vec<&'static str>) {
    const MAX_CYCLES: usize = 240;
    let mut register: i32 = 1;
    let mut cycle: usize = 0;

    let read_out_cycles: Vec<i32> = (20..=220).step_by(40).collect();
    let mut read_out_cycle_pointer = 0;
    let mut signal_strengths = vec![0; 6];

    let mut crt_signal = vec![""; MAX_CYCLES];

    for command in commands {
        match *command {
            "noop" => write_pixel(&mut crt_signal, &mut cycle, register),

            add_command => {
                    write_pixel(&mut crt_signal, &mut cycle, register);
                    write_pixel(&mut crt_signal, &mut cycle, register);

                // Read out signal strength at given cycle numbers.
                if read_out_cycle_pointer < read_out_cycles.len() && read_out_cycles[read_out_cycle_pointer] <= cycle as i32 {
                    signal_strengths[read_out_cycle_pointer] = read_out_cycles[read_out_cycle_pointer] * register;
                    read_out_cycle_pointer += 1;
                }

                register += add_command
                    .split_once(" ")
                    .map(|(_, c)| c.parse::<i32>().unwrap())
                    .unwrap();
            }
        }
    }

    return (signal_strengths.iter().sum(), crt_signal);
}

fn main() {
    let commands: Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();

    let (part_1, part_2) = execute_commands(&commands);

    println!("Part-1: {part_1}");
    println!("Part-2:");

    // Print output CRT signal
    for s in part_2.chunks(40) {
        println!("{:?}", s);
    }
}
