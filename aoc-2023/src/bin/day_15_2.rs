fn main() {
    let bytes = include_str!("../../assets/day_15.txt").as_bytes();

    // an inner Linked List is what would be correct, but all my homies hate Linked Lists, so we use a vector
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];

    let mut label: String = "".into();
    for byte in bytes {
        match byte {
            // all sequences end in either a number or a '-'
            b'\n' | b',' | b'=' => continue,
            b'-' => {
                let hash = hash_label(&label);
                let pos = boxes[hash].iter().position(|val| val.0 == label);
                if let Some(pos) = pos {
                    boxes[hash].remove(pos);
                }
                label.clear();
            }
            num if *byte >= 49 && *byte <= 57 => {
                let num = *byte - b'0';
                let hash = hash_label(&label);
                let pos = boxes[hash].iter().position(|val| val.0 == label);
                if let Some(pos) = pos {
                    boxes[hash][pos].1 = num;
                } else {
                    boxes[hash].push((label.clone(), num));
                }
                label.clear();
            }
            _ => {
                label.push(*byte as char);
            }
        }
    }

    let mut result = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (slot, val) in b.iter().enumerate() {
            result += (i + 1) * (slot + 1) * val.1 as usize;
        }
    }

    println!("RESULT: {result}");
}

fn hash_label(label: &str) -> usize {
    let mut result: usize = 0;
    for byte in label.as_bytes() {
        result += *byte as usize;
        result = (result * 17) % 256;
    }

    result
}
