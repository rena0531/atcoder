use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        t: i32,
    };
    // let n = 20;
    // let x = 12;
    // let t = 6;

    if n % x == 0 {
        println!("{}", (n / x) * t);
    } else {
        println!("{}", (n / x + 1) * t);
    }
}
