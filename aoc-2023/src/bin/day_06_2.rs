fn main() {
    let mut lines = include_str!("../../assets/day_06.txt").lines();
    let mut result = 0;

    let time: u64 =
        lines.next().unwrap().split_ascii_whitespace().skip(1).fold(String::from(""), |acc, x| acc + x).parse::<u64>().unwrap();
    let distance: u64 =
        lines.next().unwrap().split_ascii_whitespace().skip(1).fold(String::from(""), |acc, x| acc + x).parse::<u64>().unwrap();

    for ms in 0..=time {
        if (time - ms) * ms > distance {
            result += 1;
        }
    }

    println!("RESULT: {result}");
}
