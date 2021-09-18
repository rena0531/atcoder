use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: String
    };

    let mut ans: String = "".to_string();
    for i in t.chars() {
        if i == '1' {
            ans.push_str(&s1);
        } else if i == '2' {
            ans.push_str(&s2);
        } else {
            ans.push_str(&s3);
        }
    }

    println!("{}", ans);
}
