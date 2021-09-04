use proconio::input;
use std::collections::HashSet;

fn main() {
    let mut s = HashSet::new();
    for _ in 0..3 {
        input! {
            x: String
        }
        s.insert(x);
    }

    if !(s.contains("ABC")) {
        println!("ABC");
    } else if !(s.contains("ARC")) {
        println!("ARC");
    } else if !(s.contains("AGC")) {
        println!("AGC");
    } else {
        println!("AHC");
    }
}
