fn sum(seq: &[i32]) -> i32 {
    seq.iter().fold(0, |acc, current| acc + current)
}

fn max_sequence(seq: &[i32]) -> i32 {
    seq.iter().enumerate().fold(0, |acc, current| {
        let (i, _) = current;
        let x = seq[i..].iter().enumerate().fold(0, |acc, current2| {
            let (j, _) = current2;
            let arr = &seq[i..j];
            let y = sum(arr);
            if y > acc {
                return y;
            }
            return acc;
        });
        x
    })
}

fn main() {
    let r = max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    println!("{r}, 6");
}
