fn main() {}
fn gimme(input_array: [i32; 3]) -> usize {
    let max = input_array.iter().max().unwrap();
    let min = input_array.iter().min().unwrap();
    let mut middle_index: Option<usize> = None;
    for (i, n) in input_array.iter().enumerate() {
        if n != max && n != min {
            middle_index = Some(i);
        }
    }
    return middle_index.unwrap();
}

mod tests {
    use crate::gimme;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
