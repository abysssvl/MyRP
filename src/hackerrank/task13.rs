pub fn divisible_sum_pairs_calc(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    let n = n as usize;

    for i in 0..n {
        for j in (i + 1)..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[allow(dead_code, non_snake_case)]
pub fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    divisible_sum_pairs_calc(n, k, ar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_pairs_standard() {
        let ar = vec![1, 3, 2, 6, 1, 2];
        assert_eq!(divisible_sum_pairs_calc(6, 3, &ar), 5);
    }

    #[test]
    fn test_divisible_pairs_no_matches() {
        let ar = vec![1, 1, 1];
        assert_eq!(divisible_sum_pairs_calc(3, 5, &ar), 0);
    }
}