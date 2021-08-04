use prcn_lib::math::dight_sum;
use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from("10 1 2");

    input! {
        // from source,
        n: u64,
        a: u64,
        b: u64
    };

    let mut ans = 0;
    for i in 0..n + 1 {
        if a <= dight_sum(i) && b >= dight_sum(i) {
            ans += i;
        }
    }

    println!("{}", ans);
}
