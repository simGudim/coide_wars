extern crate regex;
use regex::Regex;

// pub fn anagrams(word: &str, words: &[&str]) -> Vec<String> {
   
//     let mut temp = format!(r"[{0}]", word).to_string();
//     temp += format!(r"{{{}}}", word.len()).as_str();
//     let re = Regex::new(temp.as_str()).unwrap();
//     let mut result = vec![];
//     for i in words {
//         if re.is_match(i) && word.chars().map(|i| word.contains(i)) {
//             result.push(i.to_string());
//         }
//     }
//     result
// }

// pub fn anagrams(word: &str, words: &[&str]) -> Vec<String> {
//     let mut result: Vec<String> = vec![];
//     for i in words {
//         let mut counter: i32 = 0;
//         for x in i.chars() {
//             if word.matches(x).count() == i.matches(x).count() {
//                 counter += 1;
//             } else {
//                 counter -= 1;
//             }
//         }
//         if counter as usize == word.len(){
//                 result.push(i.to_string()); 
//         }
//     }
//     result
// }

// use std::collections::HashMap;
// pub fn solve(vec: &[i32]) -> Vec<i32>{
//     let mut hash_map: HashMap<i32, i32> = HashMap::new();
//     let mut result: Vec<i32> = vec![];
//     for i in vec {
//         if hash_map.contains_key(&i) {
//             *hash_map.get_mut(i).unwrap() += 1; 
//         } else {
//             hash_map.insert(*i, 1);
//         }
//     }

//     let mut value_list: Vec<(&i32, &i32)> = hash_map.iter().collect();
//     value_list.sort_by(|a, b| b.1.cmp(a.1));
//     for i in value_list {
//         for x in 0..*i.1 {
//             result.push(*i.0)
//         }
//     }
//     result
// }

pub fn weigh_the_list(a: Vec<i64>) {
    let mut result: Vec<i32>= Vec::new();
    let half_len = a.len() / 2;
    let sum_1: i32 = a[..half_len].iter().map(|&s| s as i32).sum();
    let sum_2: i32 = a[half_len..].iter().map(|&s| s as i32).sum();
    let sum_diff = sum_1 - sum_2;

    match sum_diff {
        0 => {
            for i in 0..a.len() {
                result.push(1);
            }
        },
        x if x < 0 => {
            for i in 0..half_len {
                let mut sum_1: Vec<i32> = a[..half_len]
                    .iter()
                    .map(|&x| x as i32)
                    .collect::<Vec<i32>>();
                sum_1.retain(|&x| x != *&a[i] as i32);
                let mut diff_1 = sum_2 - sum_1.iter().sum::<i32>();
                diff_1 = diff_1 / *&a[i] as i32;
                if diff_1 + sum_1.iter().sum::<i32>() == sum_2 {
                    result.push(6);
                }
                println!("{:?}", diff_1);

                
            }
        },
        _ => println!("yes")
    }

    println!("{} {}", sum_1, sum_2);
 }