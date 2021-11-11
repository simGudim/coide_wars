
extern crate itertools;
use itertools::Itertools;

pub fn weigh_the_list(a: Vec<i64>) -> Vec<i64> {

    let mut result: Vec<i64>= Vec::new();
    for i in a.chunks(2) {
        if (i[0] < 0 && i[1] < 0) || (i[0] > 0 && i[1] > 0) {
            result.push(i[1] * -1);
            result.push(i[0]);
        } else if i[0] < 0 &&  i[1] > 0 {
            result.push(i[1]);
            result.push(i[0] * -1);
        } else if i[0] > 0 && i[1] < 0 {
            result.push(i[1] * -1);
            result.push(i[0]);
        }
    }
    result
 }


fn weigh_the_list_opt(a: Vec<i64>) -> Vec<i64> {
    a.iter()
        .tuples()
        .flat_map(|(a, b)| vec![*b, -a])
        .collect()
}


 #[cfg(test)]
 mod tests {
     use super::*;
     
     fn one_test(a: Vec<i64>) {
         let w = weigh_the_list(a.clone());
         
         let mut problems: Vec<String> = Vec::new();
         if a.len() != w.len() {
             problems.push("Dimensions don't match".to_string())
         }
         if w.contains(&0i64) {
             problems.push("All coefficients must be non-zero".to_string())
         }
         
         if a.len() == w.len() {
             let ws: i64 = (0..a.len()).map(|i| a[i]*w[i]).sum();
             if ws != 0 {
                 problems.push(format!("The weighted sum is equal to {}, must be 0", ws))
             }
         }
         
         assert_eq!(problems.len(), 0, "{}", problems.join("; ") + &".");
     }
     
     #[test]
     fn fixed_tests() {
         one_test(vec![1, 2, 3, 4, 5, 6]);
         one_test(vec![-13, 52]);
         one_test(vec![-1, 1]);
         one_test(vec![2, 2, 2, 2]);
         one_test(vec![2, 7, 3, 11, 5, 23, 47, 3]);
         one_test(vec![-1, 100, -100, 1]);
         one_test(vec![1, 1, 1, -2]);
         one_test(vec![-2, 1, 1, 1]);
     }
 }
