
// pub fn is_prime(n: u64) -> bool {
//     if n < 4 {
//         return true
//     } else if n % 2 == 0 {
//         return false
//     }
//     for i in 4..(n/2) {
//         if n % i == 0 {
//             return false
//         }
//     }
//     true
// }
pub fn is_prime(n: u64) -> bool {
    let mut i = 2;

    while i <= (n as f32).sqrt() as i32 {
        if n as i32 % i == 0 {
            return false
        }
        i += 1;
    }
    true
}


pub fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut primes = vec![];
    for i in m..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    if !primes.is_empty() {
        for i in 0..primes.len() {
            let end_check: usize = if (i + g as usize) > primes.len() - 1 { primes.len() - 1 } else { i + g as usize };
            let end = primes[i..end_check]
                .into_iter()
                .find(|&x| primes[i] as i32 - *x as i32  == g  || primes[i] as i32 - *x as i32  == g * -1);
            if let Some(end) = end {
                return Some((primes[i], *end))
            }
        }
    }
    None
}


pub fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(step(g, m, n), exp)
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics_step() {
        testing(2,100,110, Some((101, 103)));
        testing(4,100,110, Some((103, 107)));
    }

    #[test]
    fn long_test() {
        testing(8,30000,100000, Some((30089, 30097)));
        testing(11,30000,100000, None);
        testing(2,10000000,11000000, Some((10000139, 10000141)));
    }
}
