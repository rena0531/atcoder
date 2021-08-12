use proconio::input;

fn main() {
    input! {
        n: i8,
        s: String
    }
    let s = s.replace("ABC", "");
    let cnt = (n - s.len() as i8) / 3;

    println!("{}", cnt);
}
