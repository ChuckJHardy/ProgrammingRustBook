use num::*;

fn main() {
    let c = Complex { re: 1.1, im: 2.2 };
    escape_time(c, 2);
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None
}

#[test]
fn test_escape_no_time() {
    let c = Complex { re: 0.0, im: 0.0 };
    assert_eq!(escape_time(c, 5), None)
}

#[test]
fn test_escape_time() {
    let c = Complex { re: -0.5, im: -0.75 };
    assert_eq!(escape_time(c, 255), Some(6));
}
