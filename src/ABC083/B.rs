use proconio::input;
// use proconio::source::auto::AutoSource;

// 各桁の和
fn sum(i: u32) -> u32 {
    i.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
}

fn main() {
    // let source = AutoSource::from("100 4 16");

    input! {
        // from source,
        n: u32,
        a: u32,
        b: u32
    };

    let mut ans = 0;
    for i in 0..n + 1 {
        if a <= sum(i) && b >= sum(i) {
            ans += i;
        }
    }

    println!("{}", ans);
}
