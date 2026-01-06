fn main() {
    let lines = include_str!("../../assets/day_09.txt").lines();
    let mut result = 0;
    let mut prediction = 0;

    let mut values: Vec<i32> = Vec::new();
    for line in lines {
        for val in line.split_ascii_whitespace() {
            values.push(val.parse::<i32>().unwrap());
        }

        while values.iter().any(|v| *v != 0) {
            prediction += *values.last().unwrap();
            values = values.windows(2).map(|window| window[1] - window[0]).collect();
        }

        result += prediction;
        prediction = 0;
        values.clear();
    }

    println!("RESULT: {result}");
}
