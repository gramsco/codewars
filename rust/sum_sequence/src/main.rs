use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    min_value(vec![3, 1, 3, 2, 5]);
}
fn sequence_sum(start: u32, end: u32, step: u32) -> u32 {
    (start..=end)
        .step_by(step.try_into().unwrap())
        .fold(0, |acc, val| acc + val)
}

const MORSE_CODE: HashMap<&str, &str> = HashMap::new();

fn decode_morse(encoded: &str) -> String {
    encoded
        .split("   ")
        .map(|word| {
            word.split('\n')
                .map(|letter| MORSE_CODE.get(letter).unwrap().to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join(" ")
}
