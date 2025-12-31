use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../assets/day_03.txt");
    let mut result = 0u32;

    // line -> gear positions
    let mut gear_set: HashSet<(usize, usize)> = HashSet::new();
    // line -> (number, start_pos, end_pos)
    let mut number_map: HashMap<usize, Vec<(u32, usize, usize)>> = HashMap::new();
    let mut curr_number = (0, 0, 0);

    for (i, line) in input.split("\n").enumerate() {
        for (pos, ch) in line.as_bytes().iter().enumerate() {
            match ch {
                digit if digit.is_ascii_digit() => {
                    if curr_number.0 == 0 {
                        curr_number.1 = pos;
                    }
                    curr_number.0 = curr_number.0 * 10 + (digit - b'0') as u32;
                }
                b'*' => {
                    if curr_number.0 > 0 {
                        curr_number.2 = pos - 1;
                        number_map.entry(i).and_modify(|numbers| numbers.push(curr_number)).or_insert(vec![curr_number]);
                        curr_number = (0, 0, 0);
                    }
                    gear_set.insert((i, pos));
                }
                _ => {
                    if curr_number.0 > 0 {
                        curr_number.2 = pos - 1;
                        number_map.entry(i).and_modify(|numbers| numbers.push(curr_number)).or_insert(vec![curr_number]);
                        curr_number = (0, 0, 0);
                    }
                }
            }
        }

        if curr_number.0 > 0 {
            curr_number.2 = line.len() - 1;
            number_map.entry(i).and_modify(|numbers| numbers.push(curr_number)).or_insert(vec![curr_number]);
        }
        curr_number = (0, 0, 0);
    }

    let mut numbers_nearby = vec![];
    for (line, gear_pos) in gear_set {
        if line > 0
            && let Some(numbers) = number_map.get(&(line - 1))
        {
            for number in numbers {
                if (gear_pos + 1 >= number.1) && (gear_pos <= number.2 + 1) {
                    numbers_nearby.push(number.0);
                }
            }
        }

        if let Some(numbers) = number_map.get_mut(&line) {
            for number in numbers {
                if (gear_pos + 1 >= number.1) && (gear_pos <= number.2 + 1) {
                    numbers_nearby.push(number.0);
                }
            }
        }

        if let Some(numbers) = number_map.get_mut(&(line + 1)) {
            for number in numbers {
                if (gear_pos + 1 >= number.1) && (gear_pos <= number.2 + 1) {
                    numbers_nearby.push(number.0);
                }
            }
        }

        if numbers_nearby.len() == 2 {
            result += numbers_nearby[0] * numbers_nearby[1];
        }
        numbers_nearby.clear();
    }

    println!("RESULT: {result}");
}
