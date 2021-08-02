use proconio::input;

fn main() {
    input! {
        x: i32,
        a: i32,
        b: i32
    };

    // let x = 1000;
    // let a = 108;
    // let b = 108;

    let mut ans = 0;
    let i = (x - a) / b;
    if x - (a + i * b) < 0 {
        ans = 0;
    } else {
        ans = x - (a + i * b)
    }
    println!("{}", ans);
}
