use std::collections::VecDeque;

type Tile = (usize, usize, u8);

fn main() {
    let lines = include_str!("../../assets/day_16.txt").lines();
    let mut input: Vec<Vec<char>> = vec![];

    // (visited, Vec<heading_to>)
    // heading_to:
    // 0 to north
    // 1 to east
    // 2 to south
    // 3 to west
    let mut dp: Vec<Vec<(bool, Vec<u8>)>> = vec![];

    for (i, line) in lines.enumerate() {
        input.push(vec![]);
        dp.push(vec![]);
        for ch in line.chars() {
            input[i].push(ch);
            dp[i].push((false, vec![]));
        }
    }

    let rows = dp.len() - 1;
    let cols = dp[0].len() - 1;
    println!("ROWS: {rows}, COLS: {cols}");

    // (pos_x, pos_y, heading_to)
    let mut stack: VecDeque<Tile> = VecDeque::new();
    stack.push_back((0, 0, 1));
    while !stack.is_empty() {
        let tile = stack.pop_front().unwrap();

        // we have already analyzed this path
        if dp[tile.0][tile.1].1.contains(&tile.2) {
            continue;
        }

        match input[tile.0][tile.1] {
            '\\' => {
                if let Some(mirrored_tile) = mirrored_tile(tile, '\\', rows, cols) {
                    stack.push_back(mirrored_tile);
                }
            }
            '/' => {
                if let Some(mirrored_tile) = mirrored_tile(tile, '/', rows, cols) {
                    stack.push_back(mirrored_tile);
                }
            }
            '-' => {
                if tile.2 == 1 || tile.2 == 3 {
                    if let Some(next_tile) = next_tile(tile, rows, cols) {
                        stack.push_back(next_tile);
                    }
                } else {
                    let split_tiles = split_tile(tile, '-', rows, cols);
                    if let Some(tile) = split_tiles.0 {
                        stack.push_back(tile);
                    }
                    if let Some(tile) = split_tiles.1 {
                        stack.push_back(tile);
                    }
                }
            }
            '|' => {
                if tile.2 == 0 || tile.2 == 2 {
                    if let Some(next_tile) = next_tile(tile, rows, cols) {
                        stack.push_back(next_tile);
                    }
                } else {
                    let split_tiles = split_tile(tile, '|', rows, cols);
                    if let Some(tile) = split_tiles.0 {
                        stack.push_back(tile);
                    }
                    if let Some(tile) = split_tiles.1 {
                        stack.push_back(tile);
                    }
                }
            }
            _ => {
                if let Some(next_tile) = next_tile(tile, rows, cols) {
                    stack.push_back(next_tile);
                }
            }
        }

        dp[tile.0][tile.1].0 = true;
        dp[tile.0][tile.1].1.push(tile.2);
    }

    let result = dp.iter().flatten().filter(|val| val.0).count();
    println!("RESULT: {result}");
}

fn next_tile(tile: Tile, rows: usize, cols: usize) -> Option<Tile> {
    match tile.2 {
        0 => {
            if tile.0 == 0 {
                return None;
            }
            Some((tile.0 - 1, tile.1, tile.2))
        }
        1 => {
            if tile.1 == cols {
                return None;
            }
            Some((tile.0, tile.1 + 1, tile.2))
        }
        2 => {
            if tile.0 == rows {
                return None;
            }
            Some((tile.0 + 1, tile.1, tile.2))
        }
        _ => {
            if tile.1 == 0 {
                return None;
            }
            Some((tile.0, tile.1 - 1, tile.2))
        }
    }
}
fn mirrored_tile(tile: Tile, ch: char, rows: usize, cols: usize) -> Option<Tile> {
    match (ch, tile.2) {
        // go north
        ('\\', 3) | ('/', 1) => {
            if tile.0 == 0 {
                return None;
            }
            Some((tile.0 - 1, tile.1, 0))
        }
        // go east
        ('\\', 2) | ('/', 0) => {
            if tile.1 == cols {
                return None;
            }
            Some((tile.0, tile.1 + 1, 1))
        }
        // go south
        ('\\', 1) | ('/', 3) => {
            if tile.0 == rows {
                return None;
            }
            Some((tile.0 + 1, tile.1, 2))
        }
        // go west
        _ => {
            if tile.1 == 0 {
                return None;
            }
            Some((tile.0, tile.1 - 1, 3))
        }
    }
}
fn split_tile(tile: Tile, ch: char, rows: usize, cols: usize) -> (Option<Tile>, Option<Tile>) {
    if ch == '|' && (tile.2 == 1 || tile.2 == 3) {
        if tile.0 == 0 {
            return (Some((tile.0 + 1, tile.1, 2)), None);
        }
        if tile.0 == rows {
            return (Some((tile.0 - 1, tile.1, 0)), None);
        }
        return (Some((tile.0 - 1, tile.1, 0)), Some((tile.0 + 1, tile.1, 2)));
    }

    if ch == '-' && (tile.2 == 0 || tile.2 == 2) {
        if tile.1 == 0 {
            return (Some((tile.0, tile.1 + 1, 1)), None);
        }
        if tile.1 == cols {
            return (Some((tile.0, tile.1 - 1, 3)), None);
        }
        return (Some((tile.0, tile.1 - 1, 3)), Some((tile.0, tile.1 + 1, 1)));
    }

    (None, None)
}
