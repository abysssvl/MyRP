pub fn page_count_calc(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);

    std::cmp::min(from_front, from_back)
}

#[allow(dead_code, non_snake_case)]
pub fn pageCount(n: i32, p: i32) -> i32 {
    page_count_calc(n, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count_from_front() {
        assert_eq!(page_count_calc(6, 2), 1);
    }

    #[test]
    fn test_page_count_from_back() {
        assert_eq!(page_count_calc(5, 4), 0);
    }
}