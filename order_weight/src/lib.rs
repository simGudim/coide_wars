use std::collections::HashMap;

fn order_weight(s: &str) -> String {
    let mut str_split: Vec<&str> = s.split(" ").collect();
    let mut res: Vec<(usize, &str, u32)> = Vec::new();
    let mut result = String::from("");
    for i in str_split.iter().enumerate() {
        let sum = i.1.chars().scan(0 ,|state, x| {
            *state = *state + x.to_digit(10).unwrap();
            Some(*state)}).collect::<Vec<u32>>().pop().unwrap_or_else(|| 0);
        res.push((i.0, i.1, sum));
    }
    res.sort_by(|a, b| a.2.cmp(&b.2));
    let mut counter = 0i32;
    while counter < (res.len() - 1) as i32 {
        if (res[counter as usize].2 == res[counter as usize + 1].2) && (res[counter as usize].1 > res[counter as usize + 1].1) {
            let temp = res[counter as usize];
            res[counter as usize] = res[counter as usize + 1];
            res[counter as usize + 1] = temp;
            counter -= 1;
        }   
        counter += 1;
    }
    for x in res.iter().enumerate() {
        if x.0 != res.len()  - 1 {
            result.push_str(x.1.1);
            result.push_str(" ");
        } else {
            result.push_str(x.1.1);
        }
    }
    result
}
fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing("2000 10003 1234000 44444444 9999 11 11 22 123", 
        "11 11 2000 10003 22 123 1234000 44444444 9999");
    
}
