fn wave(s: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for (i, v) in s.chars().enumerate() {
        if v != ' ' {
            result.push(upper(s, i));
        }
    }
    result
}

fn upper(s: &str, index: usize) -> String {
    let r: Vec<String> = s
        .chars()
        .enumerate()
        .map(|el| {
            let (i, val) = el;
            if i == index {
                return val.to_uppercase().to_string();
            }
            return val.to_string();
        })
        .collect();
    return r.join("");
}

fn main() {
    wave("toto"); // [Toto, tOto, toTo, totO]
}
