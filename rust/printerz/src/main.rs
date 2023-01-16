const valid_letters: &str = "abcdefghijklm";

fn printer_error(s: &str) -> String {
    let errors = s.chars().fold(0, |acc, c| {
        if valid_letters.contains(c) {
            acc
        } else {
            acc + 1
        }
    });
    return format!("{errors}/{}", s.len());
}

fn longest(a1: &str, a2: &str) -> String {
    let c = a1.clone().to_string().push_str(a2);
    let d = a2.clone().to_string();
}

fn main() {
    println!("Hello, world!");
}
