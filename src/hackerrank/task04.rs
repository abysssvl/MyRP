pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple = ((grade / 5) + 1) * 5;
                if next_multiple - grade < 3 {
                    next_multiple
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[allow(dead_code, non_snake_case)]
pub fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grading_students(grades)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students_standard() {
        let grades = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&grades), expected);
    }

    #[test]
    fn test_no_rounding_for_failed() {
        let grades = vec![37, 29, 0];
        let expected = vec![37, 29, 0];
        assert_eq!(grading_students(&grades), expected);
    }

    #[test]
    fn test_exact_multiples_of_five() {
        let grades = vec![40, 45, 80];
        let expected = vec![40, 45, 80];
        assert_eq!(grading_students(&grades), expected);
    }
}