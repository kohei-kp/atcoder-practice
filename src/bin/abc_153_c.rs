use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [i64; n],
    }

    h.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    for i in 0..n {
        if i < k {
            continue;
        }
        ans += h[i];
    }
    println!("{}", ans);
}
