fn main() {
    let input = include_str!("../../assets/day_04.txt");
    let mut result = 0u32;

    // a map would probably be better for most cases but in the case of only 10 winning numbers, a simple .contains check might even outperform
    let mut winning_numbers: Vec<u8> = Vec::with_capacity(10);
    let mut curr_matches = 0;
    for card in input.split("\n") {
        for (i, blocks) in card.split(&[':', '|']).skip(1).enumerate() {
            if i == 0 {
                for number in blocks.split_ascii_whitespace() {
                    winning_numbers.push(number.parse::<u8>().unwrap());
                }
            }

            if i == 1 {
                for number in blocks.split_ascii_whitespace() {
                    if winning_numbers.contains(&number.parse::<u8>().unwrap()) {
                        curr_matches += 1;
                    }
                }
            }
        }

        if curr_matches > 0 {
            result += 2_u32.pow(curr_matches - 1);
        }
        curr_matches = 0;
        winning_numbers.clear();
    }

    println!("RESULT: {result}");
}
