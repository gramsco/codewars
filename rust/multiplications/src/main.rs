fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = vec![];
    for x in 1..=len {
        let mut t = vec![];
        for y in 1..=len {
            t.push(x * y);
        }
        res.push(t);
    }
    return res;
}

fn multiplication_table2(n: usize) -> Vec<Vec<usize>> {
    (1..=n).map(|i| (1..=n).map(|j| j * i).collect()).collect()
}

fn main() {
    println!("Hello, world!");
}
