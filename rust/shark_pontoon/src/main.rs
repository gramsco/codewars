use std::ops::Div;

fn main() {
    println!("Hello, world!");
    let r = shark(12.0, 50.0, 4.0, 8.0, true);
    println!("{}", r);
}

enum Life {
    Alive,
    Dead,
}

fn convert(life: Life) -> String {
    return match life {
        Life::Alive => "Alive!".to_string(),
        Life::Dead => "Shark Bait!".to_string(),
    };
}

fn compute(
    pontoon_distance: f64,
    shark_distance: f64,
    you_speed: f64,
    shark_speed: f64,
    dolphin: bool,
) -> Life {
    if shark_distance < pontoon_distance {
        return Life::Dead;
    }
    let how_many_seconds_to_pontoon = pontoon_distance / you_speed;
    let dolphin_coefficient: f64 = match dolphin {
        true => 1.0 / 2.0,
        false => 1.0,
    };

    println!("{}", how_many_seconds_to_pontoon);
    let how_many_seconds_for_shark_to_eat_you =
        shark_distance / (shark_speed * dolphin_coefficient);

    return match how_many_seconds_to_pontoon > how_many_seconds_for_shark_to_eat_you {
        true => Life::Dead,
        false => Life::Alive,
    };
}

fn shark(
    pontoon_distance: f64,
    shark_distance: f64,
    you_speed: f64,
    shark_speed: f64,
    dolphin: bool,
) -> String {
    let status = compute(
        pontoon_distance,
        shark_distance,
        you_speed,
        shark_speed,
        dolphin,
    );
    return convert(status);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(shark(12.0, 50.0, 4.0, 8.0, true), "Alive!");
        assert_eq!(shark(7.0, 55.0, 4.0, 16.0, true), "Alive!");
        assert_eq!(shark(24.0, 0.0, 4.0, 8.0, true), "Shark Bait!");
        assert_eq!(shark(40.0, 35.0, 3.0, 20.0, true), "Shark Bait!");
        assert_eq!(shark(7.0, 8.0, 3.0, 4.0, true), "Alive!");
    }
}
