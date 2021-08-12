use proconio::input;

fn main() {
    input! {s: String}

    let s = s.replace("eraser", "");
    let s = s.replace("erase", "");
    let s = s.replace("dreamer", "");
    let s = s.replace("dream", "");

    if s.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}
