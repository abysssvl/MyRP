pub fn bon_appetit_calc(bill: &[i32], k: i32, b: i32) -> String {
    let k = k as usize;

    // Рахуємо суму всіх страв, крім тієї, яку Анна не їла
    let total_shared: i32 = bill
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != k)
        .map(|(_, &price)| price)
        .sum();

    let anna_actual_share = total_shared / 2;

    if b == anna_actual_share {
        "Bon Appetit".to_string()
    } else {
        (b - anna_actual_share).to_string()
    }
}

#[allow(dead_code, non_snake_case)]
pub fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    println!("{}", bon_appetit_calc(bill, k, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_fair_bill() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit_calc(&bill, 1, 7), "Bon Appetit");
    }

    #[test]
    fn test_bon_appetit_overcharged() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit_calc(&bill, 1, 12), "5");
    }
}