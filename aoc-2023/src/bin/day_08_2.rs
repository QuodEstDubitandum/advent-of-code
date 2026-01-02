use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../../assets/day_08.txt").lines();

    // 1 for right, 0 for left
    let mut dirs: Vec<u8> = Vec::new();
    for char in lines.next().unwrap().chars() {
        if char == 'R' {
            dirs.push(1);
        } else {
            dirs.push(0);
        }
    }

    let mut parts: Vec<&str> = Vec::new();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let mut current_nodes: Vec<String> = Vec::new();

    lines.next().unwrap();
    for line in lines {
        parts = line.split_ascii_whitespace().collect();
        nodes.insert(parts[0].to_string(), (parts[2].replace(['(', ','], ""), parts[3].replace(')', "")));
        if parts[0].ends_with("A") {
            current_nodes.push(parts[0].to_string());
        }
    }

    let periods: Vec<u64> = current_nodes.iter().map(|n| steps_to_z(n, &dirs, &nodes)).collect();
    let result = periods.into_iter().reduce(least_common_multiple).unwrap();

    println!("RESULT: {result}");
}

fn steps_to_z(start: &str, dirs: &[u8], nodes: &HashMap<String, (String, String)>) -> u64 {
    let mut node = start.to_string();
    let mut steps = 0u64;

    while !node.ends_with("Z") {
        match dirs[steps as usize % dirs.len()] {
            0 => node = nodes.get(&node).unwrap().0.clone(),
            _ => node = nodes.get(&node).unwrap().1.clone(),
        }
        steps += 1;
    }
    steps
}

fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    a * b / greatest_common_divisor(a, b)
}
