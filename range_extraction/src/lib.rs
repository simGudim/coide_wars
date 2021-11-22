mod solution {
    
    pub fn range_extraction(a: &[i32]) -> String {
        let mut result = String::from("");
        let mut counter = 0;
        let mut continues = 0;
        let mut diff: i32;
        let mut low: &i32 = &0;
        let mut high: &i32;
        loop {
            if counter == a.len() - 1 {
                break
            }

            diff = a[counter + 1] - a[counter];
            if diff == 1 {
                if continues == 0 {
                    low = &a[counter];
                }
                continues += 1;
                counter += 1;
            } else {
                if (continues == 0) || (continues <= 1) {
                    result.push_str(&format!("{},", low));
                    result.push_str(&format!("{},", &a[counter]));
                    counter += 1;
                } else {
                    high = &a[counter];
                    result.push_str(&format!("{}-{},", low, high));
                    counter += 1;
                    continues = 0
                }
            }
        }
        println!("{}", result);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));	
        // assert_eq!("-3--1,2,10,15,16,18-20", solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));

        solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]);
        assert_eq!(true, false);
    }
}
