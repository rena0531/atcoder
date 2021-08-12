use proconio::input;

fn main() {
    input! {
        a: i8,
        b: i8,
        k: usize
    };

    let mut array: Vec<i8> = Vec::new();
    for i in 1..=100 {
        if a % i == 0 && b % i == 0 {
            array.push(i);
        }
    }

    println!("{:?}", array[array.len() - k]);
}
