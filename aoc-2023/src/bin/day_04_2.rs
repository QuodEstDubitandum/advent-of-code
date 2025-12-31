use std::collections::HashMap;

fn main() {
    let input = include_str!("../../assets/day_04.txt");
    let mut result = 0u32;

    let mut winning_numbers: Vec<u8> = Vec::with_capacity(10);
    let mut curr_matches = 0usize;
    let len = input.lines().count();
    let mut card_map: HashMap<usize, u32> = (0..len).map(|card| (card, 1)).collect::<HashMap<usize, u32>>();

    for (card, line) in input.lines().enumerate() {
        for (i, blocks) in line.split(&[':', '|']).skip(1).enumerate() {
            if i == 0 {
                for number in blocks.split_ascii_whitespace() {
                    winning_numbers.push(number.parse::<u8>().unwrap());
                }
            }

            if i == 1 {
                let curr_card_amount = card_map.get(&card).copied();
                for number in blocks.split_ascii_whitespace() {
                    if winning_numbers.contains(&number.parse::<u8>().unwrap()) {
                        curr_matches += 1;
                        card_map.entry(card + curr_matches).and_modify(|occ| *occ += curr_card_amount.unwrap());
                    }
                }
            }
        }

        curr_matches = 0;
        winning_numbers.clear();
    }

    result += card_map.values().sum::<u32>();

    println!("RESULT: {result}");
}
