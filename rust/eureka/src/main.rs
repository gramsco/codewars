fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b)
        .filter(|x| {
            x.to_string().len() == 1
                || x.to_string()
                    .chars()
                    .enumerate()
                    .map(|(i, v)| {
                        v.to_string()
                            .parse::<u64>()
                            .unwrap()
                            .pow((i + 1_usize).try_into().unwrap())
                    })
                    .fold(0, |acc, item| acc + item)
                    == *x
        })
        .collect()
}

fn main() {
    let r = sum_dig_pow(1, 90);
    println!("{r:?}");
}
