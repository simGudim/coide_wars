pub fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false
    }

    for i in 2..=((x as f64).sqrt() - 1f64) as i64 {
        if x % i == 0 {
            return false
        }
    }

    true
}