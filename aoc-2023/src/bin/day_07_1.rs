use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let lines = include_str!("../../assets/day_07.txt").lines();
    let mut chars: HashMap<char, u8> = HashMap::new();
    // maps from hand (7 for fives, 6 for fours ... 1 for high card) to the corresponding (hand, bid)
    let mut hands: HashMap<u8, Vec<(&str, u32)>> = HashMap::new();
    let mut occurances: Vec<u8> = Vec::with_capacity(5);
    let mut hand = "";
    let mut bid = "";

    for line in lines {
        (hand, bid) = line.split_once(" ").unwrap();
        for ch in hand.chars() {
            chars.entry(ch).and_modify(|occ| *occ += 1).or_insert(1);
        }

        occurances = chars.values().copied().collect();
        occurances.sort_unstable_by(|a, b| b.cmp(a));
        chars.clear();

        let rank = match occurances[0] {
            // fives
            5u8 => 7u8,
            // fours
            4u8 => 6u8,
            3u8 => {
                if occurances[1] == 2 {
                    // full house
                    5u8
                } else {
                    // threes
                    4u8
                }
            }
            2u8 => {
                if occurances[1] == 2 {
                    // double pair
                    3u8
                } else {
                    // pair
                    2u8
                }
            }
            // high card
            _ => 1u8,
        };

        hands
            .entry(rank)
            .and_modify(|v| v.push((hand, bid.parse::<u32>().unwrap())))
            .or_insert(vec![(hand, bid.parse::<u32>().unwrap())]);
    }

    let mut result = 0;
    let mut rank = 1;
    let mut current_hands: Vec<(&str, u32)> = Vec::new();
    for i in 1..=7 {
        if let Some(v) = hands.get(&i) {
            current_hands = v.clone();
            // smallest first
            current_hands.sort_unstable_by(|a, b| PokerOrdering(a.0).cmp(&PokerOrdering(b.0)));

            for curr_hand in current_hands {
                result += curr_hand.1 * rank;
                rank += 1;
            }
        }
    }

    println!("RESULT: {result}");
}

const fn make_char_rank_table() -> [u8; 256] {
    let mut table = [0; 256];

    table[b'A' as usize] = 12;
    table[b'K' as usize] = 11;
    table[b'Q' as usize] = 10;
    table[b'J' as usize] = 9;
    table[b'T' as usize] = 8;
    table[b'9' as usize] = 7;
    table[b'8' as usize] = 6;
    table[b'7' as usize] = 5;
    table[b'6' as usize] = 4;
    table[b'5' as usize] = 3;
    table[b'4' as usize] = 2;
    table[b'3' as usize] = 1;
    table[b'2' as usize] = 0;

    table
}
static CHAR_RANK: [u8; 256] = make_char_rank_table();

#[derive(Debug, Eq, PartialEq)]
struct PokerOrdering<'a>(&'a str);

impl<'a> Ord for PokerOrdering<'a> {
    fn cmp(&self, value: &Self) -> Ordering {
        let a_bytes = self.0.as_bytes();
        let b_bytes = value.0.as_bytes();
        for i in 0..a_bytes.len() {
            match (CHAR_RANK[a_bytes[i] as usize], CHAR_RANK[b_bytes[i] as usize]) {
                (a, b) if a < b => return Ordering::Less,
                (a, b) if a > b => return Ordering::Greater,
                _ => (),
            }
        }

        Ordering::Equal
    }
}

impl<'a> PartialOrd for PokerOrdering<'a> {
    fn partial_cmp(&self, value: &Self) -> Option<Ordering> {
        Some(self.cmp(value))
    }
}
