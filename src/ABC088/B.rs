use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    let source = AutoSource::from(
        "
        2
        3 1                                
    ",
    );

    input! {
        from source,
        n: i8,
        mut a: [i8; n]
    };

    // for i in 0..a.len() - 1 {
    //     for j in (i + 1..a.len()).rev() {
    //         if a[j - 1] > a[j] {
    //             a.swap(j, j - 1);
    //         }
    //

    // 降順に
    a.sort_by(|x, y| x.cmp(y).reverse());

    let mut alice_sum = 0;
    let mut bob_sum = 0;

    // for i in 0..a.len() {
    //     if a.len() % 2 != 1 {
    //         if i % 2 == 1 {
    //             alice_sum += a[i];
    //         } else {
    //             bob_sum += a[i];
    //         }
    //     } else {
    //         if i % 2 == 1 {
    //             bob_sum += a[i];
    //         } else {
    //             alice_sum += a[i];
    //         }
    //     }
    // }

    // enumerate
    // 0: 3
    // 1: 1
    for (i, &x) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice_sum += x;
        } else {
            bob_sum += x;
        }
    }

    println!("{}", alice_sum - bob_sum);
}
