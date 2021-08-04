use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // デバッグ方法
    // let source = AutoSource::from(
    //     "6
    //     382253568 723152896 37802240 379425024 404894720 471526144
    //     ",
    // );
    input! {
        // from source,
        n: usize,
        mut a: [usize; n]
    };
    let mut flg = true;
    let mut ans = 0;

    loop {
        for i in 0..n {
            if a[i] % 2 == 1 {
                flg = false;
            }
            a[i] /= 2;
        }
        if flg == true {
            ans += 1;
        } else {
            println!("{}", ans);
            break;
        }
    }
}
