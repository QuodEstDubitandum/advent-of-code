use std::collections::HashMap;

fn main() {
    let input = include_str!("../../assets/day_12.txt");

    let mut result: u64 = 0;

    for line in input.lines() {
        let (seq, nums) = line.split_once(' ').unwrap();

        let mut seq = format!("{seq}?").repeat(5);
        seq.pop();
        let sequence = seq.as_bytes();

        let mut nums = format!("{nums},").repeat(5);
        nums.pop();
        let seq_lens: Vec<usize> = nums.split(',').map(|n| n.parse().unwrap()).collect();

        let mut memo = HashMap::new();
        let ways = count_ways(sequence, &seq_lens, 0, 0, 0, &mut memo);
        result += ways;
    }

    println!("RESULT: {}", result);
}

fn count_ways(
    sequence: &[u8],
    seq_lens: &[usize],
    i: usize,
    g: usize,
    iteration: usize,
    cache: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&(i, g, iteration)) {
        return cached;
    }

    // --- end of input ---
    if i == sequence.len() {
        let valid = if iteration > 0 { g + 1 == seq_lens.len() && iteration == seq_lens[g] } else { g == seq_lens.len() };

        return valid as u64;
    }

    let mut total = 0;
    let ch = sequence[i];

    // --- place '.' ---
    if ch == b'.' || ch == b'?' {
        if iteration == 0 {
            total += count_ways(sequence, seq_lens, i + 1, g, 0, cache);
        } else if g < seq_lens.len() && iteration == seq_lens[g] {
            total += count_ways(sequence, seq_lens, i + 1, g + 1, 0, cache);
        }
    }

    // --- place '#' ---
    if (ch == b'#' || ch == b'?') && g < seq_lens.len() && iteration < seq_lens[g] {
        total += count_ways(sequence, seq_lens, i + 1, g, iteration + 1, cache);
    }

    cache.insert((i, g, iteration), total);
    total
}
