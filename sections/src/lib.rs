fn c(k: i64) -> i32 {
    let mut result = 2;
    let mut temp = vec![];
    let k3 = k as f64 * (k as f64).sqrt();
    if (k3 - k3.floor() == 0.0) && (k <= 10000000000) {
        let k4 = k3 as i64;
        let half = k3.sqrt() as i64;
        for x in 2..=half {
            if k4 % x == 0 {
                temp.push((x, k4/x));
                result += 2;
            } 
        }
        println!("{:?}", temp);
        result
    } else {
        0
    }
}
// 6923904920
//[(2, 2597847584), (4, 1298923792), (8, 649461896), (16, 324730948), (32, 162365474), (64, 81182737), (433, 11999296), (866, 5999648), (1732, 2999824), (3464, 1499912), (6928, 749956), (13856, 374978), (27712, 187489)]

#[cfg(test)]
    mod tests {
    use super::*;

    fn testing(k: i64, exp: i32) -> () {
        let ans = c(k);
        assert_eq!(ans, exp, "Testing: {}", k);
    }

    #[test]
    fn basic_tests() {
        // testing(423128, 0);
        // testing(1369, 4);
        // testing(2999824, 28);
        // testing(11710084, 64);
        // testing(25, 4);
        testing(6923904920, 0);
        // assert_eq!(true, false);   
    }
}