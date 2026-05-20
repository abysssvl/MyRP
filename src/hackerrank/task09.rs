pub fn migratory_birds_calc(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut max_id = 1;

    for id in 1..=5 {
        if counts[id] > max_count {
            max_count = counts[id];
            max_id = id as i32;
        }
    }

    max_id
}

#[allow(dead_code, non_snake_case)]
pub fn migratoryBirds(arr: &[i32]) -> i32 {
    migratory_birds_calc(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_standard() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds_calc(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_tie_break() {
        let arr = vec![1, 2, 2, 4, 4, 4, 2, 5];
        assert_eq!(migratory_birds_calc(&arr), 2);
    }
}