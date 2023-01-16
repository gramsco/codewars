fn adjacent_elements_product(xs: &[i32]) -> i32 {
    xs.windows(2).map(|x| x[0] * x[1]).max().unwrap()
}

fn main() {
    let add = |x, y| x + y;
    let add2 = |x: &[i32]| add(x[0], x[1]);
    let x = [1, 2, 3, 4, 5];
    let z = x.windows(2).map(add2).max().unwrap();
    println!("{z}");
}
