pub fn nth_prime_number(nth: u32) -> u32 {
    let mut number = 2_u32;
    let mut index = 1_u32;

    while index < nth {
        number += 1;

        if is_prime(number) {
            index += 1;
        }
    }

    number
}

fn is_prime(val: u32) -> bool {
    let upper_bound = (val as f64).sqrt().ceil() as u32;

    for x in 2..=upper_bound {
        if val % x == 0 {
            return false;
        }
    }

    true
}
