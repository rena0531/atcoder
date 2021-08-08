use proconio::input;
// use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from("3 6");

    input! {
        // from source,
        a: u32,
        b: u32
    };
    let mut ans = 0;
    for i in 0..256 {
        // ^: ビットXOR
        if i ^ a == b {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
