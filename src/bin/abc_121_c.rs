use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(i64, i64); n],
    }

    ab.sort_by(|x, y| x.0.cmp(&y.0));

    let mut ans = 0;
    let mut cnt = 0;
    for (a, b) in ab {
        if cnt + b <= m as i64 {
            ans += a * b;
            cnt += b;
        } else {
            ans += a * (m as i64 - cnt);
            break;
        }
    }
    println!("{}", ans);
}
