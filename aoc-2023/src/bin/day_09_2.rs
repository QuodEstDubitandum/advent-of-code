fn main() {
    let lines = include_str!("../../assets/day_09.txt").lines();
    let mut result = 0;
    let mut subtract = true;

    let mut values: Vec<i32> = Vec::new();
    for line in lines {
        for val in line.split_ascii_whitespace() {
            values.push(val.parse::<i32>().unwrap());
        }
        let mut prediction = *values.first().unwrap();

        while values.iter().any(|v| *v != 0) {
            values = values.windows(2).map(|window| window[1] - window[0]).collect();
            match subtract {
                true => prediction -= *values.first().unwrap(),
                false => prediction += *values.first().unwrap(),
            }
            subtract = !subtract;
        }

        result += prediction;
        subtract = true;
        values.clear();
    }

    println!("RESULT: {result}");
}
