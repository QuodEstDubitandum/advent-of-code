fn main() {
    let lines = include_str!("../../assets/day_10.txt").lines();
    let mut board: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);

    for (i, line) in board.iter().enumerate() {
        if let Some(j) = line.iter().position(|char| *char == 'S') {
            start = (i, j);
            break;
        }
    }

    let loop_positions = calc_loop_positions(&board, start);
    let loop_set: std::collections::HashSet<_> = loop_positions.iter().copied().collect();

    let s_type = determine_s_type(&board, start);
    board[start.0][start.1] = s_type;

    let mut count = 0;
    for (i, _row) in board.iter().enumerate() {
        let mut inside = false;
        let mut last_corner = ' ';

        for (j, ch) in board[i].iter().enumerate() {
            if loop_set.contains(&(i, j)) {
                match ch {
                    '|' => inside = !inside,
                    'L' | 'F' => last_corner = *ch,
                    '7' => {
                        if last_corner == 'L' {
                            inside = !inside;
                        }
                    }
                    'J' => {
                        if last_corner == 'F' {
                            inside = !inside;
                        }
                    }
                    _ => {}
                }
            } else if inside {
                count += 1;
            }
        }
    }

    println!("RESULT: {count}");
}

fn determine_s_type(board: &[Vec<char>], start: (usize, usize)) -> char {
    let connects_north = start.0 > 0 && matches!(board[start.0 - 1][start.1], '|' | '7' | 'F');
    let connects_south = start.0 < board.len() - 1 && matches!(board[start.0 + 1][start.1], '|' | 'L' | 'J');
    let connects_west = start.1 > 0 && matches!(board[start.0][start.1 - 1], '-' | 'L' | 'F');
    let connects_east = start.1 < board[0].len() - 1 && matches!(board[start.0][start.1 + 1], '-' | 'J' | '7');

    match (connects_north, connects_south, connects_west, connects_east) {
        (true, true, false, false) => '|',
        (false, false, true, true) => '-',
        (true, false, false, true) => 'L',
        (true, false, true, false) => 'J',
        (false, true, false, true) => 'F',
        (false, true, true, false) => '7',
        _ => 'S',
    }
}

fn calc_loop_positions(board: &[Vec<char>], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut loop_positions = Vec::new();
    loop_positions.push(start);

    let mut previous_position = start;
    let mut current_position = go_first_step(board, start);

    while current_position != start {
        loop_positions.push(current_position);
        let prev = current_position;
        match board[current_position.0][current_position.1] {
            '-' => {
                current_position = (current_position.0, 2 * current_position.1 - previous_position.1);
            }
            '|' => {
                current_position = (2 * current_position.0 - previous_position.0, current_position.1);
            }
            'L' => {
                if current_position.0 != previous_position.0 {
                    current_position = (current_position.0, current_position.1 + 1);
                } else {
                    current_position = (current_position.0 - 1, current_position.1);
                }
            }
            'F' => {
                if current_position.0 != previous_position.0 {
                    current_position = (current_position.0, current_position.1 + 1);
                } else {
                    current_position = (current_position.0 + 1, current_position.1);
                }
            }
            'J' => {
                if current_position.0 != previous_position.0 {
                    current_position = (current_position.0, current_position.1 - 1);
                } else {
                    current_position = (current_position.0 - 1, current_position.1);
                }
            }
            '7' => {
                if current_position.0 != previous_position.0 {
                    current_position = (current_position.0, current_position.1 - 1);
                } else {
                    current_position = (current_position.0 + 1, current_position.1);
                }
            }
            _ => (),
        }
        previous_position = prev;
    }

    loop_positions
}

fn go_first_step(board: &[Vec<char>], start: (usize, usize)) -> (usize, usize) {
    let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for comb in directions {
        if let Some(ch) =
            board.get((start.0 as i32 + comb.0) as usize).and_then(|line| line.get((start.1 as i32 + comb.1) as usize))
        {
            match (comb, ch) {
                ((0, 1), '-') | ((0, 1), 'J') | ((0, 1), '7') => return (start.0, start.1 + 1),
                ((0, -1), '-') | ((0, -1), 'F') | ((0, -1), 'L') => return (start.0, start.1 - 1),
                ((1, 0), '|') | ((1, 0), 'L') | ((1, 0), 'J') => return (start.0 + 1, start.1),
                ((-1, 0), '|') | ((-1, 0), 'F') | ((-1, 0), '7') => return (start.0 - 1, start.1),
                _ => (),
            }
        }
    }

    (0, 0)
}
