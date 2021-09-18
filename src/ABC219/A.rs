use proconio::input;

fn main() {
    input! {
        x: i32,
    };

    if x >= 0 && x < 40 {
        println!("{}", 40 - x);
    } else if x >= 40 && x < 70 {
        println!("{}", 70 - x);
    } else if x >= 70 && x < 90 {
        println!("{}", 90 - x);
    } else if x >= 90 {
        println!("expert");
    }
}
