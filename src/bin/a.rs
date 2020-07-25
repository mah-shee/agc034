#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        s: Chars,
    }
    // c < dのとき、aがbを追い越すことはないので、b,
    // aがそれぞれゴールに辿り着けるかどうかを判定すればいい

    // c > dのとき、aがbを追い越すことになる。
    // 追い越すことができる条件は、b < d の間に岩のない空間が最小3つ連続していること。
    // そのタイミングでaはbを追いこすことができ、そこからはa,
    // bのがそれぞれゴールにたどり着けるかどうかを判定する
    // ゴールにたどり着ける条件は、岩のない空間が2つ連続しないこと
}
