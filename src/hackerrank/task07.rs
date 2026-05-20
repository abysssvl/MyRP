fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

pub fn get_total_x_calc(a: &[i32], b: &[i32]) -> i32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }

    let mut lcm_a = a[0];
    for &val in a.iter().skip(1) {
        lcm_a = lcm(lcm_a, val);
    }

    let mut gcd_b = b[0];
    for &val in b.iter().skip(1) {
        gcd_b = gcd(gcd_b, val);
    }

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

#[allow(dead_code, non_snake_case)]
pub fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    get_total_x_calc(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets_standard() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        // НСК для a = 4, НСД для b = 16.
        // Числа, які кратні 4 і ділять 16: 4, 8, 16
        assert_eq!(get_total_x_calc(&a, &b), 3);
    }

    #[test]
    fn test_between_two_sets_example2() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        // НСК для a = 6, НСД для b = 12.
        // Числа: 6, 12
        assert_eq!(get_total_x_calc(&a, &b), 2);
    }
}