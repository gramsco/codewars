fn double_arr(list: &Vec<i32>) -> Vec<i32> {
    list.iter().map(|x| 2 * x).collect()
}

fn main() {
    let a = vec![1, 2, 3];
    let b = double_arr(&a);
    let c = double_arr(&b);

    println!("A : {:?}, B : {:?}, C : {:?}", a, b, c);
}
