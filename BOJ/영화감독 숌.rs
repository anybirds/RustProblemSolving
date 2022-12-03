// BOJ 1436 영화감독 숌

use std::io;

fn read_integer() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn is_title(i: i32) -> bool {
    let mut count_six = 0;
    for character in i.to_string().chars() {
        if character == '6' {
            count_six += 1;
            if count_six >= 3 {
                return true;
            }
        } else {
            count_six = 0;
        }
    }

    false
}

fn main() {
    let n = read_integer();

    let mut i = 0;
    let mut count = 0;
    loop {
        if is_title(i) {
            count += 1;
        }
        if count == n {
            break;
        }
        i += 1;
    }
    print!("{}", i);
}