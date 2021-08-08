use proconio::input;
// use proconio::source::auto::AutoSource;
use std::cmp::max;
use std::cmp::min;

fn main() {
    // let source = AutoSource::from("1500 2000 1600 3 2");

    input! {
        // from source,
        a: i32,
        b: i32,
        c: i32,
        x: i32,
        y: i32
    };

    let mut ans = std::i32::MAX;
    for i in 0..=x + y {
        let cost = 2 * c * i + a * max(0, x - i) + b * max(0, y - i);
        ans = min(ans, cost);
    }

    println!("{}", ans);
}
