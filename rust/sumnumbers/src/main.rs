fn get_sum(a: i64, b: i64) -> i64 {
    match a < b {
        true => (a..=b).sum(),
        false => (b..=a).sum(),
    }
}

fn minimum(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    let arr = [
        "I love you",
        "a little",
        "a lot",
        "passionately",
        "madly",
        "not at all",
    ];
    let index = (nb_petals as usize - 1) % arr.len();
    return arr[index];
}

fn main() {
    let a = get_sum(1, 0);
    let b = get_sum(1, 2);
    let c = get_sum(0, 1);
    let d = get_sum(1, 1);
    let e = get_sum(-1, 0);
    let f = get_sum(-1, 2);
    let g = get_sum(5, -1);
    println!(
        "(a) {} = 1, (b) {} = 3, (c) {} = 1, (d) {} = 1, (e) {} =-1, (f) {} =2 (g) {} = 14 ",
        a, b, c, d, e, f, g
    );
}
