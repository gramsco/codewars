fn invert(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .map(|x| match x.signum() {
            1 => -x,
            _ => x.abs(),
        })
        .collect()
}

enum Parity {
    Odd,
    Even,
}

fn find_outlier(values: &[i32]) -> i32 {
    let copy = values.clone().to_vec();
    let evens: Vec<&i32> = copy.iter().filter(|&x| x % 2 == 0).collect();
    let parity = match evens.len() {
        1 => Parity::Odd,
        _ => Parity::Even,
    };

    return match parity {
        Parity::Odd => **evens.first().unwrap(),
        Parity::Even => *copy.iter().find(|&x| x % 2 == 0).unwrap(),
    };
}

fn main() {
    let x = find_outlier(&[2, 6, 8, -10, 3]);
    let v = vec![1, 2, 2, 3];
    let z = v.clone().dedup();
    println!("{z:?}");
}

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut copy: Vec<T::Item> = sequence.into_iter().collect();
    copy.dedup();
    copy
}

/**
 * 
 * for (let i = 0 ;  i < 6 ; i++){
  Bomb.diffuse(42)
}

Bomb.diffuse(this.BombKey)


this.diffuseTheBomb = function(){
  return true
}
Bomb.diffuse()
Bomb.diffuse(3.14159)

function subtractYears(numOfYears, date = new Date()) {
  date.setFullYear(date.getFullYear() - numOfYears);
  return date;
}

Bomb.diffuse(subtractYears(4))

console.log(Bomb)
 */
