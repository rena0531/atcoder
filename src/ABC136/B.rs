use proconio::input;

fn sum(i: u32) -> usize {
    i.to_string().len()
}
fn main() {
    input! {
        n: u32
    };

    let mut ans = 0;
    for i in 1..=n {
        if sum(i) % 2 == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
