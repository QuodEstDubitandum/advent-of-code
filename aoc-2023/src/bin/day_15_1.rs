fn main() {
    let bytes = include_str!("../../assets/day_15.txt").as_bytes();
    let mut result = 0;

    let mut curr_val: u32 = 0;
    for byte in bytes {
        if *byte == b'\n' {
            continue;
        }
        if *byte == b',' {
            result += curr_val;
            curr_val = 0;
            continue;
        }

        curr_val += *byte as u32;
        curr_val = (curr_val * 17) % 256;
    }

    result += curr_val;

    println!("RESULT: {result}");
}
