use proconio::input;
use std::collections::{HashMap};

fn main() {
    input! {
        n: usize,
        a: [[String; 2]; n],
    };

    let mut s: Vec<String> = [].to_vec();
    let mut t: Vec<String> = [].to_vec();
    for i in 0..n {
        s.push(a[i][0].clone());
        t.push(a[i][1].clone());
    }

    let mut map = HashMap::new();
    for s in a {
        let counter = map.entry(s).or_insert(0);
        *counter += 1;
    }

    let mut ans = 0;
    for (_, count) in &map {
        if count > &1 {
            ans += 1;
        } else {
            ans += 0;
        }
    }

    if ans > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
