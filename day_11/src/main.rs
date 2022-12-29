use itertools::Itertools;

#[derive(Clone)]
struct Monkey {
    items_held: Vec<usize>,
    operation: Vec<String>,
    dividend: usize,
    true_throw_index: usize,
    false_throw_index: usize,
    inspection_counter: usize,
}

impl Monkey {
    fn new(
        items_held: Vec<usize>,
        operation: Vec<String>,
        dividend: usize,
        true_throw_index: usize,
        false_throw_index: usize,
    ) -> Monkey {
        Monkey {
            items_held,
            operation,
            dividend,
            true_throw_index,
            false_throw_index,
            inspection_counter: 0,
        }
    }
}

fn execute_operation(current_value: usize, operation: &Vec<String>, div: usize) -> usize {
    let value_2_str = &operation[2];
    let operation = &operation[1];

    let value_2 = if value_2_str == "old" { current_value } else { value_2_str.parse::<usize>().unwrap() };

    let mut new_value = if operation == "*" { current_value * value_2 } else { current_value + value_2 };

    new_value = if div == 3 {
        new_value / 3
    } else {
        new_value % div
    };

    return new_value;
}


fn parse_monkey(info_string: &str) -> Monkey {
    let info_string_vector: Vec<&str> = info_string.split("\n").collect();
    let items: Vec<usize> = info_string_vector[1]
        .split_once(": ").unwrap().1
        .split(", ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let operation: Vec<String> = info_string_vector[2]
        .split_once("= ").unwrap().1
        .split(" ")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let dividend = info_string_vector[3]
        .split(" ")
        .last().unwrap()
        .parse::<usize>().unwrap();

    let true_throw_index = info_string_vector[4]
        .split(" ")
        .collect::<Vec<&str>>().last().unwrap()
        .parse::<usize>().unwrap();

    let false_throw_index = info_string_vector[5]
        .split(" ")
        .last().unwrap()
        .parse::<usize>().unwrap();

    return Monkey::new(items, operation, dividend, true_throw_index, false_throw_index);
}

fn execute_round(monkeys: &mut Vec<Monkey>, div: usize) {
    (0..monkeys.len()).for_each(|i| {
        loop {
            let monkey = &mut monkeys[i];

            if monkey.items_held.is_empty() { break; }
            monkey.inspection_counter += 1;

            let current_item = monkey.items_held.remove(0);
            let new_item = if monkey.operation[1] == "+" {
                execute_operation(current_item, &monkey.operation, div)
            } else {
                execute_operation(current_item, &monkey.operation, div)
            };

            let test_condition = new_item % monkey.dividend == 0;
            let new_item_placement_index = if test_condition { monkey.true_throw_index } else { monkey.false_throw_index };

            monkeys[new_item_placement_index].items_held.push(new_item);
        }
    });
}

fn execute_n_rounds(monkeys: &Vec<Monkey>, round_count: usize, dividend: usize) -> usize {
    let mut monkey_copy = monkeys.clone();
    (0..round_count).for_each(|_| execute_round(&mut monkey_copy, dividend));

    return monkey_copy.iter()
        .map(|m| m.inspection_counter)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(2)
        .product::<usize>();
}


fn main() {
    let monkeys: Vec<Monkey> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| parse_monkey(s))
        .collect();

    let mut big_dividend = 1;
    for monkey in &monkeys {
        big_dividend *= monkey.dividend;
    }

    println!("Part-1: {:?}", execute_n_rounds(&monkeys, 20, 3));
    println!("Part-2: {:?}", execute_n_rounds(&monkeys, 10_000, big_dividend));
}
