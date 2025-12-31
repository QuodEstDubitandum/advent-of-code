fn main() {
    let input = include_str!("../../assets/day_02.txt");
    let mut result = 0u32;

    'games: for game in input.split("\n") {
        for draws in game.split(&[':', ',', ';']).skip(1) {
            if let Some((amount, color)) = draws.trim().split_once(" ") {
                match color {
                    "red" if amount.parse::<u8>().unwrap() > 12 => continue 'games,
                    "green" if amount.parse::<u8>().unwrap() > 13 => continue 'games,
                    "blue" if amount.parse::<u8>().unwrap() > 14 => continue 'games,
                    _ => (),
                }
            }
        }

        // IDE inserts new line at the end of txt file
        if !game.is_empty() {
            result += game.split_once(":").unwrap().0.replace("Game ", "").parse::<u8>().unwrap() as u32;
        }
    }

    println!("RESULT: {result}");
}
