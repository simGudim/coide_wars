// John and Mary want to travel between a few towns A, B, C ... Mary has on a sheet of paper a list of distances between these towns. 
// ls = [50, 55, 57, 58, 60]. John is tired of driving and he says to Mary that he doesn't want to drive more than t = 174 miles and he will visit only 3 towns.
// Which distances, hence which towns, they will choose so that the sum of the distances is the biggest possible to please Mary and John?

use std::ops::Deref;



pub fn factorial(num: &i32) -> i32 {
    match num {
        0  => 1,
        1.. => (1..num+1).product(),
        _ => unimplemented!()
    }
}

fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> Vec<Vec<&i32>> {
    // formula for combinations n!/(z!(n-z)!)
    let mut combo_list: Vec<Vec<&i32>> = vec![];
    let ls: Vec<&i32> = ls.into_iter().filter(|x| x.deref() < &172).collect::<Vec<&i32>>();
    let n:i32 = ls.len() as i32;
    let z: i32 = 3;
    let number_of_combinations = factorial(&n)/(factorial(&z)*factorial(&(n - z)));
    println!("number of combinations: {}", &number_of_combinations);
        while combo_list.len() != (number_of_combinations as usize) {
            let mut combo: Vec<&i32> = Vec::new();
            let mut i = 0;
            while combo.len() != (k as usize) {
                let elem = ls[(i as usize)];
                if !combo.contains(&elem) {
                    combo.push(&elem);
                }
                i += 1;
            }
            combo_list.push(combo);
        }
    combo_list
}

#[test]
fn test_combinations() {
    let ts = &vec![50, 55, 56, 57, 58];
    let result = choose_best_sum(163, 3, ts);
    println!("result: {:?}", result);
}


// fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
//     assert_eq!(choose_best_sum(t, k, ls), exp)
// }

// #[ignore]
// #[test]
// fn basics_choose_best_sum() {
//     let ts = &vec![50, 55, 56, 57, 58];
//     testing(163, 3, ts, 163);
//     let ts = &vec![50];
//     testing(163, 3, ts, -1);
//     let ts = &vec![91, 74, 73, 85, 73, 81, 87];
//     testing(230, 3, ts, 228);
//     testing(331, 2, ts, 178);
    
// }
