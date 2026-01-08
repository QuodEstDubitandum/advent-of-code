use std::collections::VecDeque;

fn main() {
    let lines = include_str!("../../assets/day_14.txt").lines();
    let mut board: Vec<Vec<char>> = lines.clone().map(|line| line.chars().collect()).collect();

    // keep history of board states to figure out cycle
    const HISTORY_SIZE: usize = 100;
    let mut prev_rocks: VecDeque<Vec<Vec<char>>> = VecDeque::new();
    prev_rocks.push_back(board.clone());

    for i in 0..1_000_000_000 {
        tilt_north(&mut board);
        tilt_west(&mut board);
        tilt_south(&mut board);
        tilt_east(&mut board);

        if let Some(history_pos) = prev_rocks.iter().position(|prev| *prev == board) {
            let cycle_length = prev_rocks.len() - history_pos;
            println!("Found cycle with length: {}", cycle_length);

            let remainder = (1_000_000_000 - i - 1) % cycle_length;
            board = prev_rocks[history_pos + remainder].clone();
            break;
        }

        if prev_rocks.len() > HISTORY_SIZE {
            prev_rocks.pop_front();
        }
        prev_rocks.push_back(board.clone());
    }

    let mut result = 0;
    let total_rows = lines.count();

    for (i, row) in board.iter().enumerate() {
        for ch in row {
            if *ch == 'O' {
                result += total_rows - i;
            }
        }
    }

    println!("RESULT: {result}");
}

fn tilt_north(board: &mut [Vec<char>]) {
    for j in 0..board[0].len() {
        let mut next_rock_pos = 0;
        for i in 0..board.len() {
            match board[i][j] {
                '#' => next_rock_pos = i + 1,
                'O' => {
                    board[i][j] = '.';
                    board[next_rock_pos][j] = 'O';
                    next_rock_pos += 1;
                }
                _ => (),
            }
        }
    }
}
fn tilt_west(board: &mut [Vec<char>]) {
    for i in 0..board.len() {
        let mut next_rock_pos = 0;
        for j in 0..board[0].len() {
            match board[i][j] {
                '#' => next_rock_pos = j + 1,
                'O' => {
                    board[i][j] = '.';
                    board[i][next_rock_pos] = 'O';
                    next_rock_pos += 1;
                }
                _ => (),
            }
        }
    }
}
fn tilt_south(board: &mut [Vec<char>]) {
    for j in 0..board[0].len() {
        let mut next_rock_pos: i32 = board.len() as i32 - 1;
        for i in (0..board.len()).rev() {
            match board[i][j] {
                '#' => next_rock_pos = i as i32 - 1,
                'O' => {
                    board[i][j] = '.';
                    board[next_rock_pos as usize][j] = 'O';
                    next_rock_pos -= 1;
                }
                _ => (),
            }
        }
    }
}
fn tilt_east(board: &mut [Vec<char>]) {
    for i in 0..board.len() {
        let mut next_rock_pos: i32 = board[0].len() as i32 - 1;
        for j in (0..board[0].len()).rev() {
            match board[i][j] {
                '#' => next_rock_pos = j as i32 - 1,
                'O' => {
                    board[i][j] = '.';
                    board[i][next_rock_pos as usize] = 'O';
                    next_rock_pos -= 1;
                }
                _ => (),
            }
        }
    }
}
