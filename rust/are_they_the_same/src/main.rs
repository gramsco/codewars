use std::cmp::Ordering;

fn main() {}

fn points(games: &[String]) -> u32 {
    games
        .iter()
        .fold(0, |acc, val| acc + calculate_points(val.to_string()))
}

fn calculate_points(str: String) -> u32 {
    let splitted: Vec<u32> = str.split(":").map(|x| x.parse::<u32>().unwrap()).collect();
    let y: Option<i32> = None;
    y.unwrap();
    let first = splitted.get(0).unwrap();
    let second = splitted.get(1).unwrap();
    match first.cmp(second) {
        Ordering::Greater => 3,
        Ordering::Equal => 1,
        Ordering::Less => 0,
    }
}

mod tests {
    use crate::{calculate_points, points};

    #[test]
    fn calc() {
        assert_eq!(calculate_points("3:1".to_string()), 3);
        assert_eq!(calculate_points("1:3".to_string()), 0);
        assert_eq!(calculate_points("3:3".to_string()), 1);
    }
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn toto() {
        let t = "yolo";
        println!("{t}")
    }

    fn do_fixed_test(e: &[&str], expected: u32) {
        let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(points(a), expected, "{ERR_MSG} with games = {a:?}")
    }

    #[test]
    fn sum() {
        do_fixed_test(
            &[
                "1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3",
            ],
            30,
        );
    }
}
