use std::fmt::Write;
use std::collections::VecDeque;

pub fn add_arrays(arr_a: &[i64], arr_b: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    if arr_a.is_empty() && arr_b.is_empty() {
        return result
    } else if arr_a.is_empty() && !arr_b.is_empty() {
        return arr_b.to_vec()
    } else if arr_b.is_empty() && !arr_a.is_empty() {
        return arr_a.to_vec()
    } else {
        let result_1: String = arr_a.iter().fold(String::new(), |mut s, &n| {write!(s, "{}", n).ok(); s});
        let result1 = result_1.parse::<i32>().unwrap();
        let result_2: String = arr_b.iter().fold(String::new(), |mut s, &n| {write!(s, "{}", n).ok(); s});
        let result2 = result_2.parse::<i32>().unwrap();
        let mut result_temp: VecDeque<char> = (result2 + result1).to_string().chars().collect();
        let mut i = 0;
        while i <= (result_temp.len() - 1) {
            if result_temp[i] == '-' {
                result.push((result_temp[i + 1].to_digit(12).unwrap() as i64) * -1i64);
                i+= 2;
            } else {
                result.push(result_temp[i].to_digit(12).unwrap() as i64);
                i+=1;
            }
        }
        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(add_arrays(&[6, 7], &[-6, 9]), [-2]);
        assert_eq!(add_arrays(&[6, 7], &[6, 7]), [1, 3, 4]);
        assert_eq!(add_arrays(&[3, 2, 9], &[1, 2]), [3, 4, 1]);
        assert_eq!(add_arrays(&[4, 7, 3], &[1, 2, 3]), [5, 9, 6]);
        assert_eq!(add_arrays(&[1], &[5, 7, 6]), [5, 7, 7]);
        assert_eq!(add_arrays(&[-3, 4, 2], &[3, 4, 4]), [2]);
        assert_eq!(add_arrays(&[], &[]), []);
        assert_eq!(add_arrays(&[0], &[]), [0]);
        assert_eq!(add_arrays(&[], &[1, 2]), [1, 2]);
    }
}
