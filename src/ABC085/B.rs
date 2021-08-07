use proconio::input;
// use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     3
    //     15
    //     15
    //     15
    // ",
    // );

    input! {
        // from source,
        n: i32,
        mut d: [i32; n]
    };
    d.sort_by(|x, y| x.cmp(y));
    d.dedup();
    println!("{:?}", d.len());
}
