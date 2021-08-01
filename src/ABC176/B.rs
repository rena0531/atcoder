use proconio::input;

fn main() {
    // 数字が大きすぎる場合使えないため、Stringで入力を受け取る
    // input! {
    //     n: u64
    // };
    input! {
        n: String,
    };
    let n: Vec<u32> = n.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let n_len = n.len();

    let mut sum = 0;
    for i in 0..n_len {
        sum += n[i];
    }

    if sum % 9 == 0 || sum == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
