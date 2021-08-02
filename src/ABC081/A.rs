use proconio::input;

// fn main() {
//     input! {
//         n: String,
//     };
//     let n: Vec<u32> = n.chars().map(|x| x.to_digit(10).unwrap()).collect();

//     let mut sum = 0;
//     for x in 0..(n.len()) {
//         if n[x] == 1 {
//             sum += 1;
//         }
//     }

//     println!("{}", sum);
// }

fn main() {
    input! {
        s: String
    };

    let n = s.chars().filter(|&c| c == '1').count();
    println!("{}", n);
}

