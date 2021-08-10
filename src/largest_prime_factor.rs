pub fn largest_prime_factor(x: &str) -> String {
    let mut number: Vec<u8> = x
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let sub: &mut [u8; 3] = &mut [0, 1, 0];

    let x = addition(number.as_mut_slice(), sub).expect("None returned on subtraction");
    println!("{:?}", x);

    String::from_utf8_lossy(&x).to_string()
}

/// Large number subtraction represented by u8 slices.
/// Slice parameters are reversed in order because of the subtraction algorithm.
fn subtract<'a>(minuend: &'a mut [u8], subtrahend: &mut [u8]) -> Option<&'a [u8]> {
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
        return None;
    }

    Some(minuend)
}

fn addition<'a>(first_addend: &'a mut [u8], second_addend: &mut [u8]) -> Option<&'a [u8]> {
    let mut carry = 0_u8;

    for (fa, sa) in first_addend.iter_mut().zip(second_addend) {
        if (*fa + *sa) >= 10 {
            *fa = (*fa + *sa) - 10;
            carry = 1_u8;
        } else {
            *fa = *fa + *sa;
            carry = 0_u8;
        }
    }

    if carry > 0 {
        return None;
    }

    Some(first_addend)
}
