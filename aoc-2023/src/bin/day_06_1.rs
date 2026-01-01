fn main() {
    let mut lines = include_str!("../../assets/day_06.txt").lines();
    let mut result = 1;

    let times: Vec<u32> = lines.next().unwrap().split_ascii_whitespace().skip(1).map(|num| num.parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> =
        lines.next().unwrap().split_ascii_whitespace().skip(1).map(|num| num.parse::<u32>().unwrap()).collect();

    let mut curr_race_wins = 0;
    for race in 0..times.len() {
        for ms in 0..=times[race] {
            if (times[race] - ms) * ms > distances[race] {
                curr_race_wins += 1;
            }
        }

        result *= curr_race_wins;
        curr_race_wins = 0;
    }

    println!("RESULT: {result}");
}
