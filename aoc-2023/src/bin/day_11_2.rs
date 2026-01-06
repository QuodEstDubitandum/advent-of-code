fn main() {
    let lines = include_str!("../../assets/day_11.txt").lines();
    let mut galaxy_positions = Vec::new();

    let mut rows = 0;
    let mut cols = 0;
    for (i, line) in lines.enumerate() {
        rows += 1;
        for (j, ch) in line.chars().enumerate() {
            if i == 0 {
                cols += 1;
            }

            if ch == '#' {
                galaxy_positions.push((i, j));
            }
        }
    }

    let mut empty_rows: Vec<bool> = vec![true; rows];
    let mut empty_columns: Vec<bool> = vec![true; cols];

    for galaxy in galaxy_positions.iter() {
        empty_rows[galaxy.0] = false;
        empty_columns[galaxy.1] = false;
    }

    let mut result: i64 = 0;
    for i in 0..galaxy_positions.len() {
        for j in i + 1..galaxy_positions.len() {
            let row_diff = (galaxy_positions[j].0 as i32 - galaxy_positions[i].0 as i32).abs();
            let col_diff = (galaxy_positions[j].1 as i32 - galaxy_positions[i].1 as i32).abs();

            result += (row_diff + col_diff) as i64;
            empty_rows.iter().skip(galaxy_positions[i].0).take(row_diff as usize).for_each(|is_empty| {
                is_empty.then(|| {
                    result += 999_999;
                });
            });

            let skip_by = galaxy_positions[j].1.min(galaxy_positions[i].1);
            empty_columns.iter().skip(skip_by).take(col_diff as usize).for_each(|is_empty| {
                is_empty.then(|| {
                    result += 999_999;
                });
            });
        }
    }

    println!("RESULT: {result}");
}
