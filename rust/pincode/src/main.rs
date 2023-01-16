fn main() {
    let a = "π3π4";
    let b: Vec<usize> = a
        .to_string()
        .chars()
        .enumerate()
        .map(|(ind, _)| ind)
        .collect();
    println!("{b:?}");
}
