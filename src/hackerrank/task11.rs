pub fn diagonal_difference_calc(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        secondary_sum += arr[i][n - 1 - i];
    }

    (primary_sum - secondary_sum).abs()
}

#[allow(dead_code, non_snake_case)]
pub fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    diagonal_difference_calc(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_standard() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference_calc(&arr), 15);
    }

    #[test]
    fn test_diagonal_difference_symmetric() {
        let arr = vec![
            vec![1, 2],
            vec![2, 1],
        ];
        assert_eq!(diagonal_difference_calc(&arr), 2);
    }
}