pub fn largest_prime_factor(x: &str) -> String {
    let mut number = number_string_to_slice(x, None);

    use std::collections::HashMap;
    let mut prime_factors: HashMap<u64, u64> = HashMap::new();

    let mut factor = 2_u64;
    while factor.to_string() != x {
        let mut divisor = number_string_to_slice(&factor.to_string(), Some(number.len()));
        println!("{:?}", divisor);

        if !is_prime(factor) {
            factor += 1;
            continue;
        }

        match division(number.as_mut_slice(), &mut divisor) {
            None => factor += 1,
            Some(_) => {
                subtract(number.as_mut_slice(), &mut divisor);
                prime_factors.insert(factor, prime_factors.get(&factor).unwrap_or(&0) + 1);
            }
        }
        println!("Maplen -> {}", prime_factors.len());
    }

    if prime_factors.len() == 0 {
        return format!("Number {} is prime!", x);
    }

    for x in prime_factors.keys() {
        println!("Factor -> {}", *x)
    }

    "Rez".to_string()
}

fn number_string_to_slice(x: &str, length: Option<usize>) -> Vec<u8> {
    let number: Vec<u8> = x
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    if let Some(len) = length {
        let mut result: Vec<u8> = vec![0; len];
        for (n, r) in number.iter().zip(result.iter_mut()) {
            *r = *n;
        }

        return result;
    }

    number
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
        if (*fa + *sa + carry) >= 10 {
            *fa = (*fa + *sa + carry) - 10;
            carry = 1_u8;
        } else {
            *fa = *fa + *sa + carry;
            carry = 0_u8;
        }
    }

    if carry > 0 {
        return None;
    }

    Some(first_addend)
}

fn division(dividend: &mut [u8], divisor: &mut [u8]) -> Option<Vec<u8>> {
    let mut result: Vec<u8> = vec![0; dividend.len()];
    let mut unit: Vec<u8> = vec![0; dividend.len()];
    unit[0] = 1;

    loop {
        match subtract(dividend, divisor) {
            None => return None,
            Some(x) => {
                result = addition(result.as_mut_slice(), unit.as_mut_slice())
                    .unwrap()
                    .to_vec();

                if x.iter().fold(true, |acc, elem| acc && !(*elem > 0)) {
                    return Some(result);
                }
            }
        }
    }
}

fn is_prime(val: u64) -> bool {
    let upper_bound = (val as f64).sqrt().ceil() as u64;

    for x in 2..=upper_bound {
        if val % x == 0 {
            return false;
        }
    }

    true
}
