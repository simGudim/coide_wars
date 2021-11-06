use std::cmp::Ordering;

fn max_ball(v0: i32) -> i32 {
    let mut h = vec![];
    let mut v0 = v0 * (1000/3600);
    for i in 0..=10 {
        h.push((i, (v0 as f64)*(i as f64/10f64)-0.5*9.81*(i as f64/10f64) * (i as f64/10f64)));
    }
    let index_of_max: Option<usize> = h
        .iter()
        .enumerate()
        .max_by(|(_, b), (_, a)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index);
    println!("{:?}", h);
    println!("{:?}", &index_of_max);
    index_of_max.unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::max_ball;
    
    #[test]
    fn basic() {
        assert_eq!(max_ball(37), 10);
        assert_eq!(max_ball(45), 13);
    }    
}
