// BOJ 1259 펠린드롬수

use std::io;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line.");
        
        let n: u32 = line.trim().parse().expect("Failed to parse.");
        if n == 0 {
            break;
        }

        let chars: Vec<char> = line.trim().chars().collect();
        let mut palindrome = true;
        for i in 0..chars.len() {
            if chars[i] != chars[chars.len() - i - 1] {
                palindrome = false;
                break;
            }
        }
        if palindrome {
            println!("yes");
        } else  {
            println!("no");
        }
    }
    
}