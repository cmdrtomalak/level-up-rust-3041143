use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnError {
    InputTooLong,
    InputTooShort,
    FailedChecksum,
}

impl FromStr for Isbn {
    type Err = IsbnError; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Vec<u8> = s
                                .chars()
                                .filter(|a| a.is_numeric())
                                .map(|c| c.to_digit(10).unwrap() as u8)
                                .collect();

        if digits.len() > 13 {
            return Err(IsbnError::InputTooLong);
        }
        if digits.len() < 13 {
            return Err(IsbnError::InputTooShort);
        }

        // checksum
        let chk_digit = calculate_check_digit(&digits);

        if digits[12] != chk_digit {
            return Err(IsbnError::FailedChecksum);
        }

        Ok(
            Isbn {
            raw: s.to_string(),
            digits,
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut chk: Vec<u8> = Vec::with_capacity(digits.len());
    for i in 0..12 {
        if i % 2 == 0 {
            chk.push(digits[i] * 1);
        } else {
            chk.push(digits[i] * 3);
        }
    }

    let checksum: u8 = chk.iter().sum();
    let check_digit = (10 - (checksum % 10)) % 10;
    check_digit
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
