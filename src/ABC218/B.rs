use proconio::input;

fn main() {
    input! {
        p: [usize;26]
    };

    let v = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();

    let mut ans = String::new();
    for i in p {
        ans.push_str(&v[i - 1].to_string());
    }
    println!("{}", ans);
}
