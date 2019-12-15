use std::env;
use std::ops::RangeInclusive;

fn main() {
    let range = determine_range();
    let mut passwords: Vec<u32> = range.filter(|password| fits_facts(*password)).collect();
    println!("There are {} matching passwords", passwords.len());
    passwords.retain(|password| fits_additional_rule(*password));
    println!("Only {} meet all criteria", passwords.len());
}

fn determine_range() -> RangeInclusive<u32> {
    let mut args = env::args()
        .skip(1)
        .take(2)
        .map(|number| number.parse::<u32>().unwrap());
    let start = args.next().expect("Missing range arguments");
    let end = args.next().expect("Missing end of range");
    start..=end
}

fn fits_facts(candidate: u32) -> bool {
    let digits: Vec<u32> = to_digits(candidate);
    digits.windows(2).any(|digits| digits[0] == digits[1])
        && (0..digits.len() - 1).all(|i| digits[i] <= digits[i + 1])
}

fn fits_additional_rule(candidate: u32) -> bool {
    let digits = to_digits(candidate);
    let last_index = digits.len() - 1;
    digits
        .windows(4)
        .any(|digits| digits[0] != digits[1] && digits[1] == digits[2] && digits[2] != digits[3])
        || digits[0] == digits[1] && digits[1] != digits[2]
        || digits[last_index] == digits[last_index - 1]
            && digits[last_index - 1] != digits[last_index - 2]
}

fn to_digits(number: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut number = number;
    while number > 10 {
        result.push(number % 10);
        number /= 10;
    }
    result.push(number);
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_111111() {
        assert!(fits_facts(111_111))
    }

    #[test]
    fn rejects_123789() {
        assert!(!fits_facts(123_789))
    }

    #[test]
    fn reject_223450() {
        assert!(!fits_facts(223_450))
    }

    #[test]
    fn digits_of_122345() {
        assert_eq!(vec![1, 2, 2, 3, 4, 5], to_digits(122_345))
    }

    #[test]
    fn additional_criterion_matches_112233() {
        assert!(fits_additional_rule(112_233))
    }

    #[test]
    fn additional_criterion_rejects_123444() {
        assert!(!fits_additional_rule(123_444))
    }
}
