use std::env;
use std::str;
// valid = 1112223339

fn main() {
    let mut x = 0;

    loop {
        x += 1;
        if x > 10 {
            break;
        }
    }

    let mut r = (0..10).enumerate();

    loop {
        let s = r.next();
        println!("{:?}", s);
    }
    let isbn = read_isbn();
    match valid_isbn10(&isbn) {
        true => {
            println!("VALID ISBN 10");
            std::process::exit(0);
        }
        false => {
            println!("INVALID ISBN 10");
            std::process::exit(1);
        }
    }
}

fn read_isbn() -> String {
    let args: Vec<String> = env::args().collect();
    let query = &args.get(1);
    match query {
        Some(q) => q.to_string(),
        None => {
            println!("Please provide a ISBN.");
            std::process::exit(1);
        }
    }
}

fn valid_isbn10(isbn: &str) -> bool {
    let has_10_chars = isbn.len() == 10;
    if !has_10_chars {
        return false;
    }

    let iteration: Option<usize> = isbn.chars().enumerate().fold(Some(0), |acc, x| match acc {
        Some(acc) => {
            let (index, ch) = x;
            let value = match x {
                (9, 'X') => Ok(10),
                _ => ch.to_string().parse(),
            };
            match value {
                Ok(v) => Some(acc + (v * (index + 1))),
                _ => None,
            }
        }
        None => None,
    });

    return match iteration {
        Some(sum) => sum % 11 == 0,
        _ => false,
    };
}
