// BOJ 1157 단어 공부

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line.");
    let line = line.trim();

    let mut counts = [0; 'z' as usize - 'a' as usize + 1];
    for character in line.chars() {
        counts[character.to_ascii_lowercase() as usize - 'a' as usize] += 1;
    }
    let mut index = 0;
    let mut duplicate = false;
    for i in 1..counts.len() {
        if counts[index] < counts[i] {
            index = i;
            duplicate = false;
        } else if counts[index] == counts[i] {
            duplicate = true;
        }
    }
    if duplicate {
        print!("?");
    } else {
        print!("{}", char::from_u32('A' as u32 + index as u32).unwrap());
    }
}