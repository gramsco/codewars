fn expanded_form(n: u64) -> String {
    let mut final_str: Vec<String> = vec![];

    for (i, char) in n.to_string().chars().rev().enumerate() {
        match char == '0' {
            false => {
                let digit = char.to_string().parse::<i64>().unwrap();
                let multiplied = digit * 10_i64.pow(i.try_into().unwrap());
                final_str.push(multiplied.to_string());
            }
            true => (),
        }
    }
    final_str.reverse();
    return final_str.join(" + ");
}

fn main() {
    let r = expanded_form(2048);
    println!("{r}");
}
