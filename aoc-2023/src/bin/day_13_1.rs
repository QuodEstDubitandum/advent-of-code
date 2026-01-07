fn main() {
    let lines = include_str!("../../assets/day_13.txt").lines();

    let mut result = 0;
    let mut hashtag_matrix = Vec::new();
    for line in lines {
        if line.is_empty() {
            result += 100 * reflect_rows(&hashtag_matrix);
            result += reflect_cols(&hashtag_matrix);
            hashtag_matrix.clear();
            continue;
        }

        let mut row = Vec::new();
        for ch in line.chars() {
            match ch {
                '#' => row.push(true),
                _ => row.push(false),
            }
        }

        hashtag_matrix.push(row);
    }

    result += 100 * reflect_rows(&hashtag_matrix);
    result += reflect_cols(&hashtag_matrix);

    println!("RESULT: {result}");
}

fn reflect_rows(matrix: &[Vec<bool>]) -> usize {
    'outer: for i in 0..matrix.len() - 1 {
        let mut reflection_size = 0;
        if matrix[i] == matrix[i + 1] {
            loop {
                // we reached out of bounds, either top or bottom
                if i < reflection_size || i + reflection_size + 1 > matrix.len() - 1 {
                    return i + 1;
                }

                if matrix[i - reflection_size] == matrix[i + 1 + reflection_size] {
                    reflection_size += 1;
                } else {
                    continue 'outer;
                }
            }
        }
    }

    0
}
fn reflect_cols(matrix: &[Vec<bool>]) -> usize {
    'outer: for j in 0..matrix[0].len() - 1 {
        let mut reflection_size = 0;
        if matrix.iter().all(|row| row[j] == row[j + 1]) {
            loop {
                // we reached out of bounds, either left or right
                if j < reflection_size || j + 1 + reflection_size > matrix[0].len() - 1 {
                    return j + 1;
                }

                if matrix.iter().all(|row| row[j - reflection_size] == row[j + 1 + reflection_size]) {
                    reflection_size += 1;
                } else {
                    continue 'outer;
                }
            }
        }
    }

    0
}
