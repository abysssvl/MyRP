use std::collections::HashSet;

pub fn sock_merchant_calc(n: i32, ar: &[i32]) -> i32 {
    let mut pairs = 0;
    let mut unpaired_socks = HashSet::new();

    for &sock in ar.iter().take(n as usize) {
        if unpaired_socks.contains(&sock) {
            unpaired_socks.remove(&sock);
            pairs += 1;
        } else {
            unpaired_socks.insert(sock);
        }
    }

    pairs
}

#[allow(dead_code, non_snake_case)]
pub fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    sock_merchant_calc(n, ar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_standard() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant_calc(9, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_no_pairs() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sock_merchant_calc(5, &ar), 0);
    }
}