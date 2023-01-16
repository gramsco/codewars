fn parse(code: &str) -> Vec<i32> {
    let (arr, _) = code.chars().fold((vec![], 0_i32), |acc, val| {
        let (mut arr, mut value) = acc;
        match val {
            'i' => value = value + 1,
            'd' => value = value - 1,
            's' => value = value.pow(2),
            'o' => arr.push(value),
            _ => (),
        };
        return (arr, value);
    });
    return arr;
}

fn main() {
    let r = parse("iiisdoso");
    println!("{r:?}");
}
