fn main() {
    let input = include_str!("../../assets/day_02.txt");
    let mut result = 0u32;

    let mut rgb: (u32, u32, u32) = (0, 0, 0);
    for game in input.split("\n") {
        for draws in game.split(&[':', ',', ';']).skip(1) {
            if let Some((amount, color)) = draws.trim().split_once(" ") {
                match color {
                    "red" => rgb.0 = rgb.0.max(amount.parse::<u32>().unwrap()),
                    "green" => rgb.1 = rgb.1.max(amount.parse::<u32>().unwrap()),
                    "blue" => rgb.2 = rgb.2.max(amount.parse::<u32>().unwrap()),
                    _ => (),
                }
            }
        }

        result += rgb.0 * rgb.1 * rgb.2;
        rgb = (0, 0, 0);
    }

    println!("RESULT: {result}");
}
