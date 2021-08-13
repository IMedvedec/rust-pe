/// Function returns the largest prime factor for the given value.
/// If the given value is 0 or 1, the function panics with a message.
pub fn largest_prime_factor(val: u64) -> u64 {
    use std::collections::btree_map::BTreeMap;
    let mut prime_factors: BTreeMap<u64, u64> = BTreeMap::new();

    let mut mut_val = val;
    let mut prime_iterator = 2_u64;
    while prime_iterator <= mut_val {
        if !is_prime(prime_iterator) {
            prime_iterator += 1;
            continue;
        }

        if mut_val % prime_iterator == 0 {
            let prime_counter = prime_factors.entry(prime_iterator).or_insert(0);
            mut_val /= prime_iterator;
            *prime_counter += 1;
        } else {
            prime_iterator += 1;
        }
    }

    *prime_factors
        .keys()
        .last()
        .expect(format!("Largest prime not found for number: {}", val).as_str())
}

/// Helper function to check if a number is prime.
fn is_prime(val: u64) -> bool {
    match val {
        0 | 1 => false,
        2 => true,
        _ => {
            let upper_bound = (val as f64).sqrt().ceil() as u64;
            for x in 2..=upper_bound {
                if val % x == 0 {
                    return false;
                }
            }
            true
        }
    }
}
