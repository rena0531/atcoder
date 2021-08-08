use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! { n:u64 };

    let mut ans = std::usize::MAX;
    for i in 1..=n {
        // 最小値は必ずn以下になるため。これがないとTLE
        if i * i > n {
            break;
        }
        if n % i == 0 {
            let a = n / i;
            ans = min(ans, max(a.to_string().len(), i.to_string().len()));
        }
    }
    println!("{}", ans);
}
