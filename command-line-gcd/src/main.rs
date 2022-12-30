use std::str::FromStr;
use std::fmt;

fn main() {
    gcd(2, 2);
    gcd_from_collection(vec!["2", "2"]).expect("");
}

#[derive(Debug, PartialEq)]
enum ArgumentError {
    ParseError(String),
    EmptyError,
}

impl std::error::Error for ArgumentError {}
impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgumentError::ParseError(s) => write!(f, "Unable to parse passed arguments: {}", s),
            ArgumentError::EmptyError => write!(f, "No arguments were passed"),
        }
    }
}

fn gcd_from_collection(args: Vec<&str>) -> Result<(Vec<u64>, u64), ArgumentError> {
    let mut numbers = Vec::new();

    for arg in args {
        if let Ok(parsed) = u64::from_str(arg) {
            numbers.push(parsed);
        } else {
            return Err(ArgumentError::ParseError(arg.to_string()))
        }
    }

    if numbers.len() == 0 {
        return Err(ArgumentError::EmptyError)
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    Ok((numbers, d))
}

#[test]
fn test_gcd_from_collection() {
    assert_eq!(gcd_from_collection(vec!["2", "4"]), Ok((vec![2, 4], 2)));
    assert_eq!(gcd_from_collection(vec!["4", "8"]), Ok((vec![4, 8], 4)));
}
#[test]
fn test_gcd_from_collection_empty_args_passed() {
    assert_eq!(gcd_from_collection(vec![]), Err(ArgumentError::EmptyError));
}

#[test]
fn test_gcd_from_collection_strings_not_passed() {
    assert_eq!(gcd_from_collection(vec!["a", "b"]), Err(ArgumentError::ParseError("a".to_string())));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd_positive_path() {
    assert_eq!(gcd(2, 4), 2);
    assert_eq!(gcd(4, 8), 4);
    assert_eq!(gcd(2, 3333), 1);
}

#[test]
#[should_panic]
fn test_gcd_negative_values_past() {
    gcd(0, 0);
}

#[test]
#[should_panic]
fn test_gcd_negative_value_past() {
    gcd(1, 0);
}
