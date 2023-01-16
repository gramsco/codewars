fn fake_bin(s: &str) -> String {
    s.chars().map(|c| if c < '5' { '0' } else { '1' }).collect()
}

fn main() {
    println!("{}", fake_bin("123456789"));
}
