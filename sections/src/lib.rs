

fn c(k: i64) -> i32 {
    let mut result = 0;
    let mut temp  = vec![];
    let k3 = k as f64 * (k as f64).sqrt();
    if k3 - k3.floor() == 0.0 {
        let k4 = k3 as i64;
        if k4 % 2 == 0 {
            let half = k3.sqrt() as i64;
            for x in (2..half).step_by(2) {
                if k4 % x == 0 {
                    temp.push((x, k4/x));
                    result += 2;
                } 
            }
        } else {
            let half = k3.sqrt() as i64;
            for x in (3..half).step_by(2) {
                if k4 % x == 0 {
                    temp.push((x, k4/x));
                    result += 2;
                } 
            }
        }
        result
    } else {
        0
    }
}


#[cfg(test)]
    mod tests {
    use super::*;

    fn testing(k: i64, exp: i32) -> () {
        let ans = c(k);
        assert_eq!(ans, exp, "Testing: {}", k);
    }

    #[test]
    fn basic_tests() {
        testing(423128, 0);
        testing(1369, 4);
        testing(2999824, 28);
        // // testing(11710084, 64);
        testing(25, 4);
        
    }
}
