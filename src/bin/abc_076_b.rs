use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = 1;

    for i in 0..n {
        let ans_a = ans * 2;
        let ans_b = ans + k;
        if ans_a > ans_b {
            ans = ans_b;
        } else {
            ans = ans_a;
        }
    }

    println!("{}", ans);
}
