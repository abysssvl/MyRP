pub fn count_apples_and_oranges_calc(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges
        .iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

#[allow(dead_code, non_snake_case)]
pub fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let (apple_count, orange_count) = count_apples_and_oranges_calc(s, t, a, b, apples, oranges);
    println!("{}", apple_count);
    println!("{}", orange_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apple_and_orange_standard() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        assert_eq!(count_apples_and_oranges_calc(s, t, a, b, &apples, &oranges), (1, 1));
    }

    #[test]
    fn test_no_fruits_landing_on_house() {
        let s = 7;
        let t = 11;
        let a = 1;
        let b = 20;
        let apples = vec![1, 2];
        let oranges = vec![-1, -2];

        assert_eq!(count_apples_and_oranges_calc(s, t, a, b, &apples, &oranges), (0, 0));
    }
}