use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64
    };
    // let x = 10000;
    // let y = 10000;

    if x * y % 2 != 0 {
        println!("Odd");
    } else {
        println!("Even");
    }
}
