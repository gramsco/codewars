#[derive(PartialEq, Eq)]
enum Parity {
    Odd,
    Even,
}

fn is_odd_or_even(u: usize) -> Parity {
    match u % 2 {
        0 => Parity::Even,
        _ => Parity::Odd,
    }
}

fn invert_from(s: &str, from: Parity) -> String {
    let mut result = "".to_string();
    for (i, c) in s.chars().enumerate() {
        let status = is_odd_or_even(i);
        match status == from {
            true => result.push(c.to_ascii_uppercase()),
            false => result.push(c.to_ascii_lowercase()),
        }
    }
    return result;
}

fn capitalize(s: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let invert1 = invert_from(s, Parity::Even);
    let invert2 = invert_from(s, Parity::Odd);
    result.push(invert1);
    result.push(invert2);
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::capitalize;
    #[test]
    fn one_string() {
        assert_eq!(capitalize("yolo"), ["YoLo", "yOlO"]);
    }
    #[test]
    fn another_string() {
        assert_eq!(capitalize("hello"), ["HeLlO", "hElLo"]);
        // assert_eq!(alternate("hello"))
        // let second = test.get(1).unwrap();
        // assert_eq!(first, "HeLlO");
        // assert_eq!(second, "hElLo");
    }
}
