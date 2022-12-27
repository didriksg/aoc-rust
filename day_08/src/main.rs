fn create_range(start: usize, end: usize, rev: bool) -> Vec<usize> {
    let correct_range = if !rev {
        (start..end).collect::<Vec<usize>>()
    } else {
        (start..end).rev().collect::<Vec<usize>>()
    };

    return correct_range;
}


fn traverse(mapped_matrix: &Vec<Vec<i32>>, status_matrix: &mut Vec<Vec<bool>>, total_size: usize, reverse: bool, right_to_left: bool) {
    let outer_iter = create_range(0, total_size, reverse);
    let inner_iter = create_range(0, total_size, reverse);

    for x in outer_iter {
        let mut current_largest: i32 = -1;

        for y in &inner_iter {
            let (correct_x, correct_y) = if right_to_left {(x, *y)} else { (*y,x) };
            let current_value = mapped_matrix[correct_x][correct_y];

            current_largest = if current_value > current_largest {
                status_matrix[correct_x][correct_y] = true;
                current_value
            } else {
                current_largest
            }
        }
    }
}

fn part_1(mapped_matrix: &Vec<Vec<i32>>) -> usize {
    let matrix_size = mapped_matrix.len();
    let mut status_matrix: Vec<Vec<bool>> = vec![vec![false; matrix_size]; matrix_size];

    traverse(&mapped_matrix, &mut status_matrix, matrix_size, false, false);
    traverse(&mapped_matrix, &mut status_matrix, matrix_size, true, false);
    traverse(&mapped_matrix, &mut status_matrix, matrix_size,  false, true);
    traverse(&mapped_matrix, &mut status_matrix, matrix_size, true, true);

    return status_matrix
        .iter()
        .map(|l| l.iter()
            .filter(|x| **x)
            .count())
        .sum();
}


fn part_2(mapped_matrix: &Vec<Vec<i32>>) -> usize {
    let matrix_size = mapped_matrix.len();
    let mut max_visibility_score = 0;

    for x in 1..matrix_size - 1 {
        for y in 1..matrix_size - 1 {
            let current_value = mapped_matrix[x][y];
            let mut moving_x = x;
            let mut moving_y = y;

            let mut down_visibility_score = 0;
            let mut up_visibility_score = 0;
            let mut right_visibility_score = 0;
            let mut left_visibility_score = 0;

            while moving_x >= 1 {
                moving_x -= 1;
                down_visibility_score += 1;

                let moving_down_value = mapped_matrix[moving_x][y];
                if moving_down_value >= current_value {
                    break;
                }
            }
            moving_x = x;

            while moving_x < matrix_size - 1 {
                moving_x += 1;
                up_visibility_score += 1;

                let moving_up_value = mapped_matrix[moving_x][y];
                if moving_up_value >= current_value {
                    break;
                }
            }

            while moving_y >= 1 {
                moving_y -= 1;
                left_visibility_score += 1;


                let moving_left_value = mapped_matrix[x][moving_y];
                if moving_left_value >= current_value {
                    break;
                }
            }
            moving_y = y;

            while moving_y < matrix_size - 1 {
                moving_y += 1;
                right_visibility_score += 1;

                let moving_down_value = mapped_matrix[x][moving_y];
                if moving_down_value >= current_value {
                    break;
                }
            }

            let visibility_score = up_visibility_score * down_visibility_score * left_visibility_score * right_visibility_score;
            max_visibility_score = if visibility_score > max_visibility_score {
                visibility_score
            } else {
                max_visibility_score
            };
        }
    }

    return max_visibility_score;
}


fn main() {
    let mapped_matrix: Vec<Vec<i32>> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
    ).collect();

    let part_1_result = part_1(&mapped_matrix);
    let part_2_result = part_2(&mapped_matrix);

    println!("Part-1 result: {part_1_result}");
    println!("Part-2 result: {part_2_result}");
}
