pub fn main() {
    let shared_iter = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>());

    let part_1 = shared_iter.clone()
        .max()
        .unwrap();

    let mut item_vector = shared_iter.clone().collect::<Vec<u32>>();
    item_vector.sort_unstable();
    let part_2 = item_vector.into_iter().rev().take(3).sum::<u32>();

    println!("Part-1: {part_1}");
    println!("Part-2: {part_2}");
}
