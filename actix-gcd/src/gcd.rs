pub fn gcd(mut n: u64, mut m: u64) -> u64 {
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
