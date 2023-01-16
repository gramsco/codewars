use std::string;

fn main() {
    // let t1 = next_bigger_number(12);
    // let t2 = next_bigger_number(513);
    // let t3 = next_bigger_number(2017);
    let t4 = next_bigger_number(9876543210);
    println!("{t4}");
}

fn next_bigger_number(n: i64) -> i64 {
    let s = n.to_string();
    let mut candidate = -1;
    for (i, _c) in s.as_bytes().iter().enumerate().rev() {
        println!("{candidate}");
        let mut m: Vec<_> = s.chars().collect();
        let mut j = 0;
        loop {
            if i >= j {
                m.swap(i, i - j);
                let t: String = m.iter().collect();
                let t: i64 = t.parse().unwrap();
                if t > n && (t < candidate || candidate == -1) {
                    println!("T : {t}");
                    candidate = t;
                }
                j += 1;
            } else {
                break;
            }
        }
    }
    candidate
}

// fn permute(n: i64) -> Option<i64> {
//     let mut permutations = vec![];
//     let mut as_arr: Vec<char> = n.to_string().chars().collect();
//     let size = as_arr.len();
//     let mut value = i64::MAX;
//     heap_permutation(&mut as_arr, size, &mut |val| {
//         let l = value.to_string().len();
//         if val.len() > l {
//             return;
//         }
//         let r = val.iter().collect::<String>().parse::<i64>().unwrap();
//         if r > n && r < value {
//             println!()
//             permutations.push(r.clone());
//             value = r
//         }
//     });

//     permutations.sort();
//     permutations.get(0).map(|x| *x)
// }

// fn heap_permutation<T>(data: &mut [T], size: usize, callback: &mut dyn FnMut(&[T])) {
//     if size == 1 {
//         callback(data);
//         return;
//     }
//     for i in 0..size {
//         heap_permutation(data, size - 1, callback);
//         if size % 2 == 0 {
//             data.swap(i, size - 1);
//         } else {
//             data.swap(0, size - 1);
//         }
//     }
// }
