// Functional power of two digit sum using char iterators.
// Function workes only up to u32 overflow values of 2 on given power.
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
