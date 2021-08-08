use proconio::input;

fn main() {
    input! {
        n: i32
    };

    let mut cnt = 0;
    for i in 0..=n {
        for j in 0..=n {
            if 4 * i + 7 * j == n {
                cnt += 1;
            }
        }
    }
    if cnt > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
