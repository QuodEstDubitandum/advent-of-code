fn main() {
    let input = include_str!("../../assets/day_05.txt");
    let mut lines = input.lines();
    let mut source: Vec<u64> =
        lines.next().unwrap().split_ascii_whitespace().skip(1).map(|seed| seed.parse::<u64>().unwrap()).collect();
    source.sort_unstable();
    let mut destination: Vec<u64> = source.to_vec();

    let mut numbers: Vec<u64> = Vec::with_capacity(3);
    let mut sorted_pos = 0usize;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.contains(":") {
            destination.sort_unstable();
            source.copy_from_slice(&destination);
            continue;
        }

        numbers = line.split_ascii_whitespace().map(|num| num.parse::<u64>().unwrap()).collect();
        sorted_pos = match source.binary_search(&numbers[1]) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        for i in 0..source.len() {
            if i >= sorted_pos && source[i] <= numbers[1] + numbers[2] {
                destination[i] = source[i] - numbers[1] + numbers[0];
            }
        }
    }

    let result = destination.iter().min().unwrap();
    println!("RESULT: {result}");
}
