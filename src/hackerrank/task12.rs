pub fn birthday_cake_candles_calc(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    let &max_height = candles.iter().max().unwrap();

    candles.iter().filter(|&&h| h == max_height).count() as i32
}

#[allow(dead_code, non_snake_case)]
pub fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    birthday_cake_candles_calc(candles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candles_standard() {
        let candles = vec![4, 4, 1, 3];
        assert_eq!(birthday_cake_candles_calc(&candles), 2);
    }

    #[test]
    fn test_candles_all_same_height() {
        let candles = vec![3, 3, 3, 3];
        assert_eq!(birthday_cake_candles_calc(&candles), 4);
    }
}