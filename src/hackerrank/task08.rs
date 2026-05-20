pub fn breaking_records_calc(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut highest_count = 0;
    let mut lowest_count = 0;

    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            highest_count += 1;
        } else if score < lowest {
            lowest = score;
            lowest_count += 1;
        }
    }

    vec![highest_count, lowest_count]
}

#[allow(dead_code, non_snake_case)]
pub fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    breaking_records_calc(scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_standard() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records_calc(&scores), vec![2, 4]);
    }

    #[test]
    fn test_breaking_records_example2() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records_calc(&scores), vec![4, 0]);
    }
}