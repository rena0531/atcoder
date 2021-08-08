use proconio::input;
// use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from("4");

    input! {
        // from source,
        n: i32,
    };

    let mut ans = 0;

    for i in 1..=n {
        if i % 2 == 0 {
            continue;
        }
        let mut cnt = 0;
        for j in 1..=i {
            if i % j == 0 {
                cnt += 1;
            }
        }
        if cnt == 8 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
