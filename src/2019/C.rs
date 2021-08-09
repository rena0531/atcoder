use proconio::input;
use proconio::source::auto::AutoSource;
use std::cmp::max;

fn main() {
    let source = AutoSource::from(
        "
        3 4
        37 29 70 41
        85 69 76 50
        53 10 95 100        
        ",
    );
    input! {
        from source,
        n: usize, m: usize,
        a: [[i64; m]; n],
    }
    let mut ans = 0;
    for i in 0..m {
        for j in i + 1..m {
            let mut sum = 0;
            for k in 0..n {
                sum += max(a[k][i], a[k][j]);
                ans = max(ans, sum);
            }
        }
    }
    println!("{}", ans);
}
