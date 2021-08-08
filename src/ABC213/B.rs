use proconio::input;

fn main() {
    input! {
        n: i64,
        mut a: [i64; n]
    };

    let b = a.clone();
    a.sort_by(|x, y| x.cmp(y));
    a.reverse();
    for (i, &x) in b.iter().enumerate() {
        if a[1] == x {
            println!("{}", i + 1);
        }
    }
}
