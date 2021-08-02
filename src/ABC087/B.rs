use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32
    }
    // let a = 30;
    // let b = 40;
    // let c = 50;
    // let x = 6000;

    let mut n = 0;
    for i in 0..a + 1 {
        for j in 0..b + 1 {
            for k in 0..c + 1 {
                if i * 500 + j * 100 + k * 50 == x {
                    n += 1;
                }
            }
        }
    }
    println!("{}", n);
}
