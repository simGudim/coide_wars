

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut even: Vec<(usize, i32)> = Vec::new();
    let mut odd: Vec<(usize, i32)> = Vec::new();
    let mut result: Vec<i32> = vec![];
    if arr.is_empty() {
        return result
    }
    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            even.push((i, arr[i]));
        } else {
            odd.push((i, arr[i]));
        }
    }
    odd.sort_by(|a, b| b.1.cmp(&a.1));
    even.sort_by(|a, b| b.0.cmp(&a.0));
    for i in 0..arr.len() {
        if !even.is_empty() || even.last().unwrap().0 == i{
            let cur = even.pop().unwrap();
            result.push(cur.1);
        } else if !odd.is_empty() {
            let cur = odd.pop().unwrap();
            result.push(cur.1);
        } else {
            continue
        }
    }

    println!("{:?}", result);
    result
}