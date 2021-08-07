/// Functional power of two digit sum using char iterators.
/// Function workes only up to u32 overflow values of 2 on given power.
pub fn power_of_two_digit_sum_functional(power: u32) -> u32 {
    2_u32
        .checked_pow(power)
        .expect("Given power overflows u32 type!")
        .to_string()
        .chars()
        .map(|x| x.to_string())
        .map(|x| x.parse::<u32>().unwrap_or(0))
        .sum()
}

/// Calculate the sum of digits of the two to the power number.
/// Funciton works with large power numbers.
pub fn power_of_two_digit_sum_regular(power: u32) -> u32 {
    let digit_number: usize = (1_f64 + power as f64 * 2_f64.log10()).floor() as usize;
    let mut number_arr = vec![0_u8; digit_number];
    number_arr[0] = 1;

    for _ in 0..power {
        let mut carry = 0;

        for i in 0..digit_number {
            let new_val = number_arr[i] * 2 + carry;

            number_arr[i] = new_val % 10;
            carry = new_val / 10;
        }
    }

    number_arr.into_iter().map(|x| x as u32).sum()
}
