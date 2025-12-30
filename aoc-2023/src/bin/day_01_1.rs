fn main() {
    let input = include_str!("../../assets/day_01.txt");
    let mut result = 0u32;

    let mut curr_char: u8;
    let mut curr_line: &[u8];
    for line in input.split("\n") {
        curr_line = line.as_bytes();
        // find first digit
        for ch in curr_line {
            curr_char = ch - b'0';
            if curr_char > 0 && curr_char < 10 {
                result += (curr_char * 10) as u32;
                break;
            }
        }

        // find last digit
        for ch in curr_line.iter().rev() {
            curr_char = ch - b'0';
            if curr_char > 0 && curr_char < 10 {
                result += (curr_char) as u32;
                break;
            }
        }
    }

    println!("RESULT: {result}");
}
