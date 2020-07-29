#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        _: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        mut s: Chars,
    }

    if c > d && between(b, d, &s) == false {
        // c > dのとき、aがbを追い越すことになる。
        // 追い越すことができる条件は、b < d の間に岩のない空間が最小3つ連続していること。
        println!("No");
        return;
    }
    // そのタイミングでaはbを追いこすことができ、
    // そこからa, bのがそれぞれゴールにたどり着けるかどうかを判定する
    if goal(b, d, &s) == true && goal(a, c, &s) == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
// ゴールにたどり着ける条件は、現在いる地点からゴールの間に岩のない空間が2つ連続しないこと
fn goal(start: usize, goal: usize, map: &Vec<char>) -> bool {
    for i in start - 1..goal {
        if map[i] == '#' && map[i + 1] == '#' {
            return false;
        }
    }
    true
}

fn between(b: usize, d: usize, map: &Vec<char>) -> bool {
    for i in b - 1..d {
        if map[i - 1] == '.' && map[i] == '.' && map[i + 1] == '.' {
            return true;
        }
    }
    false
}
