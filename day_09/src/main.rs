use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
        }
    }
}


fn move_tail(head: &Coordinate, tail: &mut Coordinate) {
    if (head.x > tail.x + 1 && head.y > tail.y) || (head.x > tail.x && head.y == tail.y + 2) {
        tail.x += 1;
        tail.y += 1;
    }

    if (head.x < tail.x - 1 && head.y > tail.y) || (head.x < tail.x && head.y == tail.y + 2) {
        tail.x -= 1;
        tail.y += 1;
    }

    if (head.x < tail.x && head.y < tail.y - 1) || (head.x == tail.x - 2 && head.y < tail.y) {
        tail.x -= 1;
        tail.y -= 1;
    }


    if (head.x > tail.x + 1 && head.y < tail.y) || (head.x > tail.x && head.y == tail.y - 2) {
        tail.x += 1;
        tail.y -= 1;
    }

    if head.x > tail.x + 1 { tail.x += 1 }
    if head.x < tail.x - 1 { tail.x -= 1 }
    if head.y > tail.y + 1 { tail.y += 1 }
    if head.y < tail.y - 1 { tail.y -= 1 }
}


fn move_head(iterator: &Vec<(&str, usize)>, coordinate_count: usize) -> usize {
    let mut unique_coords: HashSet<Coordinate> = HashSet::new();
    let mut coordinates = vec![Coordinate::new(); coordinate_count];

    for (direction, count) in iterator {
        for _ in 0..*count {
            match *direction {
                "R" => coordinates[0].x += 1,
                "L" => coordinates[0].x -= 1,
                "U" => coordinates[0].y += 1,
                "D" => coordinates[0].y -= 1,
                _ => ()
            };

            for i in 1..coordinates.len() {
                let mut tail = coordinates[i].clone();
                move_tail(&coordinates[i - 1], &mut tail);
                coordinates[i] = tail;

                if i == coordinates.len() - 1 {
                    unique_coords.insert(coordinates[i].clone());
                }
            }
        }
    }

    return unique_coords.len();
}

fn main() {
    let input_lines: Vec<(&str, usize)> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(d, c)| (d, c.parse::<usize>().unwrap()))
        .collect();

    let part_1_result = move_head(&input_lines, 2);
    let part_2_result = move_head(&input_lines, 10);

    println!("Part-1: {part_1_result}");
    println!("Part-2: {part_2_result}");
}
