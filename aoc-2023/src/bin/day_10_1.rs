fn main() {
    let lines = include_str!("../../assets/day_10.txt").lines();
    let board: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);

    for (i, line) in board.iter().enumerate() {
        if let Some(j) = line.iter().position(|char| *char == 'S') {
            start = (i, j);
            break;
        }
    }

    let mut previous_position = start;
    let mut current_position = go_first_step(&board, start);
    let mut result = 1;

    while board[current_position.0][current_position.1] != 'S' {
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
        result += 1;
    }

    println!("RESULT: {}", result / 2);
}

fn go_first_step(board: &[Vec<char>], start: (usize, usize)) -> (usize, usize) {
    let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for comb in directions {
        // we know S is not at the edge of the board by looking at puzzle... otherwise you should handle this properly
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
