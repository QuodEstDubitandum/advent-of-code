fn main() {
    let lines = include_str!("../../assets/day_14.txt").lines();

    // we save the row positions that are blocked as well as if its a round rock
    let mut rocks: Vec<Vec<(bool, usize)>> = Vec::new();
    for (i, line) in lines.clone().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if i == 0 {
                rocks.push(vec![]);
            }
            match ch {
                '#' => rocks[j].push((false, i)),
                'O' => match rocks[j].last().copied() {
                    Some(last) => rocks[j].push((true, last.1 + 1)),
                    None => rocks[j].push((true, 0)),
                },
                _ => (),
            }
        }
    }

    let mut result = 0;
    let total_rows = lines.count();

    for col in rocks {
        for row_pos in col {
            if row_pos.0 {
                result += total_rows - row_pos.1;
            }
        }
    }

    println!("RESULT: {result}");
}
