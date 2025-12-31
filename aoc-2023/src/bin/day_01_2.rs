fn main() {
    let input = include_str!("../../assets/day_01.txt");
    let mut result = 0u32;

    // stores letters with positions until which these letters are currently matching
    let mut potential_letters: Vec<(&[char], usize)> = Vec::new();
    let mut curr_sum = 0;
    let mut potential_second_digit = 0;
    for line in input.split("\n") {
        'char_loop: for ch in line.chars() {
            // its a digit
            if ch.is_ascii_digit() {
                potential_letters.clear();
                if curr_sum == 0 {
                    curr_sum += ch.to_digit(10).unwrap();
                    continue 'char_loop;
                } else {
                    potential_second_digit = ch.to_digit(10).unwrap();
                    continue 'char_loop;
                }
            }

            // its not a digit and the next char is matching
            potential_letters.retain(|letter| Some(ch).as_ref() == letter.0.get(letter.1 + 1));
            for letter in potential_letters.iter_mut() {
                // full letter matching
                letter.1 += 1;
                if letter.0.len() == letter.1 + 1 {
                    if curr_sum == 0 {
                        curr_sum += map_chars_to_digit(letter.0);
                    } else {
                        potential_second_digit = map_chars_to_digit(letter.0);
                    }
                }
            }

            // in case we have new letters starting
            match ch {
                'o' => {
                    potential_letters.push((&['o', 'n', 'e'], 0));
                }
                't' => {
                    potential_letters.push((&['t', 'w', 'o'], 0));
                    potential_letters.push((&['t', 'h', 'r', 'e', 'e'], 0));
                }
                'f' => {
                    potential_letters.push((&['f', 'o', 'u', 'r'], 0));
                    potential_letters.push((&['f', 'i', 'v', 'e'], 0));
                }
                's' => {
                    potential_letters.push((&['s', 'i', 'x'], 0));
                    potential_letters.push((&['s', 'e', 'v', 'e', 'n'], 0));
                }
                'e' => {
                    potential_letters.push((&['e', 'i', 'g', 'h', 't'], 0));
                }
                'n' => {
                    potential_letters.push((&['n', 'i', 'n', 'e'], 0));
                }
                _ => (),
            };
        }

        if potential_second_digit > 0 {
            curr_sum = curr_sum * 10 + potential_second_digit;
        } else {
            curr_sum += curr_sum * 10;
        }
        result += curr_sum;
        curr_sum = 0;
        potential_second_digit = 0;
        potential_letters.clear();
    }

    println!("RESULT: {result}");
}

fn map_chars_to_digit(chars: &[char]) -> u32 {
    match chars.iter().collect::<String>().as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}
