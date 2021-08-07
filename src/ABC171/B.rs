use proconio::input;
// use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from("
    // 1 1
    // 1000
    // ");

    input! {
        // from source,
        n: i32,
        k: usize,
        mut p: [i32; n]
    };

    // 昇順に
    p.sort_by(|x, y| x.cmp(y));
    let mut sum = 0;
    p.iter().enumerate().for_each(|(i, &x)| {
        if i < k {
            sum += &x
        }
    });

    println!("{}", sum);
}
