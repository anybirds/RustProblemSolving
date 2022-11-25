use std::io;

fn main() {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Failed to read line.");
    let slices = str.split_whitespace();
    let mut sum = 0;
    for slice in slices {
        let num: u32 = slice.parse().expect("Not a number.");
        sum += num;
    }
    println!("{}", sum);
}
