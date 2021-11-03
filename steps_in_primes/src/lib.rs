
pub fn is_prime(n: i64) -> bool {
    for i in 2..(n - 1) {
        if n % i == 0 {
            return false
        }
    }
    true
}


pub fn step(g: i32, m: i64, n: i64) -> Option<(i64, i64)> {
    let result = (0, 0);
    let mut primes = vec![];
    for i in m..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    println!("{:?}", primes);
    if !primes.is_empty() {
        for i in primes.clone().iter() {
            let end = &primes.clone().into_iter().find(|x| i - x  == g as i64 || i -x == g as i64 * -1);
            if let Some(end) = end {
                return Some((*i, *end))
            }
        }
    }
    None
}


pub fn testing(g: i32, m: i64, n: i64, exp: Option<(i64, i64)>) -> () {
    assert_eq!(step(g, m, n), exp)
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics_step() {
        // assert_eq!(is_prime(101), true);
        testing(2,100,110, Some((101, 103)));
        // testing(4,100,110, Some((103, 107)));
        // testing(8,30000,100000, Some((30089, 30097)));
        // testing(11,30000,100000, None);
        // testing(2,10000000,11000000, Some((10000139, 10000141)));
    }
}
