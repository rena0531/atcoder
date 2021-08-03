use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // デバッグ方法
    let source = AutoSource::from("3
    8 12 40");
    input! {
        from source,
        n: i32,
        a: [i32; n]
    };
    println!("{}, {:?}", n, a);
    // let mut flg = true;
    // let ans = 0;

    // for i in 0..n {
    //     if a[i] % 2 != 0 {
    //         flg = false;
    //     } else {
    //         a[i] >= 1;
    //     }
    // }
    // if flg == true {
    //     ans += 1;
    // }

    // println!();
}
