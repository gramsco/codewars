use std::u8;

fn main() {
    println!("Hello, world!");
}

struct Walker {
    coordinates: Coordinates,
    steps: u8,
}

struct Coordinates {
    x: u8,
    y: u8,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn is_valid_walk(walk: &[char]) -> bool {
    true
}

#[test]
fn sample_tests() {
    assert!(is_valid_walk(&[
        'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
    ]));
    // assert!(!is_valid_walk(&[
    //     'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
    // ]));
    // assert!(!is_valid_walk(&['w']));
    // assert!(!is_valid_walk(&[
    //     'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
    // ]));
    // assert!(!is_valid_walk(&[
    //     'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
    // ]))
}
