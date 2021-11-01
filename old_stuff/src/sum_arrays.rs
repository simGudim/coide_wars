use std::fmt::Write;
use std::collections::VecDeque;

pub fn add_arrays(arr_a: &[i64], arr_b: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let result_1: String = arr_a.iter().fold(String::new(), |mut s, &n| {write!(s, "{}", n).ok(); s});
    let result1 = result_1.parse::<i32>().unwrap();
    let result_2: String = arr_b.iter().fold(String::new(), |mut s, &n| {write!(s, "{}", n).ok(); s});
    let result2 = result_2.parse::<i32>().unwrap();
    let mut result_temp: VecDeque<char> = (result2 + result1).to_string().chars().collect();
    let mut i = 0;
    while !result_temp.is_empty() {
        if result_temp[i] == '-' {
            result.push((result_temp[i + 1].to_digit(9).unwrap() as i64) * -1i64);
            result_temp.pop_front();
            result_temp.pop_front();
            i+= 2;
        } else {
            result.push(result_temp[i].to_digit(9).unwrap() as i64);
            result_temp.pop_front();
            i+=1;
        }
    }
    result
}