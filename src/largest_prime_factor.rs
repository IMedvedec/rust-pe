pub fn largest_prime_factor(x: &str) -> String {
    let mut number: Vec<u8> = x
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let sub: &mut [u8; 6] = &mut [3, 2, 1, 0, 0, 0];

    let x = subtract(number.as_mut_slice(), sub);

    String::from_utf8_lossy(&x).to_string()
}

/// Large number subtraction represented by u8 slices.
/// Slice parameters are reversed in order because of the subtraction algorithm.
fn subtract<'a>(minuend: &'a mut [u8], subtrahend: &mut [u8]) -> &'a [u8] {
    let mut carry = 0_u8;

    for (m, s) in minuend.iter_mut().zip(subtrahend) {
        if *m >= (*s + carry) {
            *m -= *s + carry;
            carry = 0_u8;
        } else {
            *m = *m + 10 - (*s + carry);
            carry = 1_u8;
        }
    }

    if carry > 0 {
        for m in minuend.iter_mut() {
            *m = 0
        }
    }

    minuend
}
