pub fn generate_staircase(n: i32) -> String {
    let n = n as usize;
    let mut result = String::new();

    for i in 1..=n {
        result.push_str(&format!("{:>width$}\n", "#".repeat(i), width = n));
    }

    result
}

#[allow(dead_code)]
pub fn staircase(n: i32) {
    print!("{}", generate_staircase(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_size_4() {
        let expected = "   #\n  ##\n ###\n####\n";
        assert_eq!(generate_staircase(4), expected);
    }

    #[test]
    fn test_staircase_size_1() {
        let expected = "#\n";
        assert_eq!(generate_staircase(1), expected);
    }

    #[test]
    fn test_staircase_size_6() {
        let expected = "     #\n    ##\n   ###\n  ####\n #####\n######\n";
        assert_eq!(generate_staircase(6), expected);
    }
}