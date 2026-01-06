use std::cmp::{max, min};

fn main() {
    let input = include_str!("../../assets/day_05.txt");
    let mut lines = input.lines();

    // including (start, end) intervals
    let mut source: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks_exact(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect();
    let mut destination: Vec<(u64, u64)> = source.clone();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.contains(":") {
            source = destination.clone();
            continue;
        }

        let numbers: Vec<u64> = line.split_ascii_whitespace().map(|num| num.parse::<u64>().unwrap()).collect();
        for i in 0..source.len() {
            let start = max(source[i].0, numbers[1]);
            let end = min(source[i].1, numbers[1] + numbers[2] - 1);

            if end < start {
                continue;
            }

            // whole interval gets mapped
            if source[i].0 == start && source[i].1 == end {
                destination[i] = (start + numbers[0] - numbers[1], end + numbers[0] - numbers[1]);
            }

            // left part gets mapped
            if source[i].0 == start && source[i].1 != end {
                destination[i] = (start + numbers[0] - numbers[1], end + numbers[0] - numbers[1]);
                destination.push((end + 1, source[i].1));
            }

            // right part gets mapped
            if source[i].0 != start && source[i].1 == end {
                destination[i] = (start + numbers[0] - numbers[1], end + numbers[0] - numbers[1]);
                destination.push((source[i].0, start - 1));
            }

            // center interval gets mapped
            if source[i].0 != start && source[i].1 != end {
                destination[i] = (start + numbers[0] - numbers[1], end + numbers[0] - numbers[1]);
                destination.push((source[i].0, start - 1));
                destination.push((end + 1, source[i].1));
            }
        }
    }

    let result = destination.iter().map(|tup| tup.0).min().unwrap();
    println!("RESULT: {result}");
}
