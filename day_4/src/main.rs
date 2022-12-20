fn main() {
    let common_iterator = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (first, second) = s.split_once(",").unwrap();
            let (start_1, stop_1) = first.split_once("-").unwrap();
            let (start_2, stop_2) = second.split_once("-").unwrap();
            (
                start_1.parse::<i16>().unwrap(),
                stop_1.parse::<i16>().unwrap(),
                start_2.parse::<i16>().unwrap(),
                stop_2.parse::<i16>().unwrap(),
            )
        });

    let mut fully_overlapped: u16 = 0;
    let mut partly_overlapped: u16 = 0;

    for (start_1, end_1, start_2, end_2) in common_iterator {
        partly_overlapped += !(end_1 - start_2 < 0 || end_2 - start_1 < 0) as u16;
        fully_overlapped += ((start_1 >= start_2 && end_1 <= end_2) || (start_1 <= start_2 && end_1 >= end_2)) as u16;
    }

    println!("Part-1 ans: {fully_overlapped}");
    println!("Part-2 ans: {partly_overlapped}");
}
