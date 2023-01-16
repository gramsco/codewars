use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

type StrI32 = (String, i32);

fn convert(code_and_quantity: String) -> StrI32 {
    let splitted: Vec<String> = code_and_quantity
        .split(" ")
        .map(|s| s.to_string())
        .collect();
    let code = splitted.get(0).unwrap().chars().nth(0).unwrap();
    let quantity = splitted.get(1).unwrap().parse::<i32>().unwrap();
    return (code.to_string(), quantity);
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return "".to_string();
    }
    let l: Vec<(String, i32)> = list_cat.iter().map(|x| (x.to_string(), 0)).collect();
    let mut map: HashMap<String, i32> = HashMap::new();

    for tuple in l {
        let (code, quantity) = tuple;
        map.insert(code, quantity);
    }

    for l in list_art.iter() {
        let (code, quantity) = convert(l.to_string());
        _ = match map.get_mut(&code) {
            Some(v) => *v += quantity,
            _ => (),
        };
    }

    let mut result = String::new();

    for &code in list_cat.iter() {
        let quantity = map.get(code).unwrap();
        result.push_str(&format!(
            "({} : {})",
            code.to_string(),
            quantity.to_string()
        ));
    }

    return result.replace(")(", ") - (");
}

mod tests {
    use crate::stock_list;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

        b = vec![];
        c = vec![];
        dotest(b, c, "");
    }
}
