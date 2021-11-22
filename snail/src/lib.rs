
fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut result = Vec::new();
    if matrix.len() > 0 {
        let mut xback = matrix[0].len();
        let mut yback = matrix.len();
        let mut area = &xback * &yback;
        let mut xfront = 0;
        let mut yfront = 0;

        while xback > 0 && yback > 0 {
            for i in xfront..xback {
                result.push(matrix[xfront][i]);
                area -= 1;
                if area  == 0{
                    break
                }
            }
            yfront += 1;
    
            for i in yfront..yback {
                println!("{}", matrix[i][yback-1]);
                result.push(matrix[i][yback-1]);
                if area  == 0{
                    break
                }
            }
            xback -= 1;

            for i in (xfront..xback).rev() {
                result.push(matrix[yback-1][i]);
                if area  == 0{
                    break
                }
            }
            yback -= 1;

            for i in (yfront..yback).rev() {
                result.push(matrix[i][xfront]);
                if area  == 0{
                    break
                }
            }
            xfront += 1;
        }
     }
    result
}


#[cfg(test)]
mod tests {
    use super::snail;

    #[test]
    fn sample_test1() {
        let square = &[
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        let expected = vec![1,2,3,6,9,8,7,4,5];
        assert_eq!(snail(square), expected);
    }
    
    #[test]
    fn sample_test2() {
        let square = &[
            vec![1,2,3],
            vec![8,9,4],
            vec![7,6,5],
        ];
        let expected = vec![1,2,3,4,5,6,7,8,9];
        assert_eq!(snail(square), expected);
    }
    
    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }
    
    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
