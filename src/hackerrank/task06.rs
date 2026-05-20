pub fn kangaroo_calc(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 > v2 && (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[allow(dead_code)]
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    kangaroo_calc(x1, v1, x2, v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_can_catch_up() {
        assert_eq!(kangaroo_calc(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_never_catches_up() {
        assert_eq!(kangaroo_calc(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_kangaroo_same_speed_different_start() {
        assert_eq!(kangaroo_calc(0, 2, 4, 2), "NO");
    }
}