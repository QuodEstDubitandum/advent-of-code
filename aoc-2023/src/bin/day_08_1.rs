use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../../assets/day_08.txt").lines();
    let mut result = 0;

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

    lines.next().unwrap();
    for line in lines {
        parts = line.split_ascii_whitespace().collect();
        nodes.insert(parts[0].to_string(), (parts[2].replace(['(', ','], ""), parts[3].replace(')', "")));
    }

    let mut current_node: String = "AAA".into();
    let last_node: String = "ZZZ".into();

    while current_node != last_node {
        match dirs[result % dirs.len()] {
            0 => current_node = nodes.get(&current_node).unwrap().0.clone(),
            _ => current_node = nodes.get(&current_node).unwrap().1.clone(),
        }
        result += 1;
    }

    println!("RESULT: {result}");
}
