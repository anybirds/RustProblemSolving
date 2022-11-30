use std::io;

fn read_integer() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}


fn main() {
    let n = read_integer();

    let mut cases = Vec::new();
    for i in 0..9 {
        if i == 6 {
            for j in 0..9 {
                if j == 6 {
                    for k in 0..9 {
                        cases.push((660 + k).to_string());
                    }
                } else {
                    cases.push((60 + j).to_string());
                }
            }
        } else {
            cases.push(i.to_string());
        }
    }

    let mut ans = String::new();
    let mut dp = vec![0, 0, 0];
    let mut count = 0;
    let mut i = 3_usize;
    loop {
        for case in &cases {
            let next;
            if case == "666" {
                next = count + 10_i32.pow(i as u32 - 3);
                if n <= next {
                    ans += case;
                    if i > 3 {
                        let remainder = (n - count).to_string();
                        ans += remainder.as_str();
                    }
                    print!("{}", ans);
                    return;
                }
            } else {
                next = count + dp[i - case.len()];
                if n <= next {
                    ans += case;
                    i -= case.len() + 1;
                    break;
                }
            }
            count = next;
        }
        if dp.len() == i {
            dp.push(count);
        }
        i += 1;
    }
}