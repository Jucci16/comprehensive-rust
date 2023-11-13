// TODO: remove this when you're done with your implementation.

pub fn luhn(cc_number: &str) -> bool {
    let formatted_number = &cc_number.replace(" ", "");
    if formatted_number.is_empty() {
        return false;
    }

    if !is_numeric(&formatted_number) {
        return false;
    }

    return is_valid_checksum(&formatted_number);
}

fn is_numeric(cc_number: &str) -> bool {
    let iter = cc_number.chars();
    for c in iter {
        if !c.is_numeric() || c.to_digit(10).unwrap() > 9 {
            return false;
        }
    }

    return true;
}

fn is_valid_checksum(cc_number: &str) -> bool {
    let iter = cc_number.chars();
    let mut j = -1;
    let mut sum = 0;
    for c in iter.rev() {
        j += 1;
        let mut int: u32 = c.to_digit(10).unwrap();

        if j % 2 == 0 {
            sum += int;
            continue;
        }

        int = if int * 2 >= 10 { int * 2 - 9 } else { int * 2 };
        sum += int;
    }

    sum % 10 == 0 && j > 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    luhn("4263 9826 4026 9299");
    println!("Running!!!");
}
