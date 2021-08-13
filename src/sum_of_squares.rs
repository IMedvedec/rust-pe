pub fn sum_of_squares(val: u64) -> u64 {
    let sequence = 1..=val;

    sequence
        .into_iter()
        .map(|x| largest_perfect_square_division(x))
        .sum()
}

fn largest_perfect_square_division(val: u64) -> u64 {
    let mut perfect_square_iter = PerfectSquareIter::new();

    let mut result = 1_u64;
    loop {
        let next = perfect_square_iter
            .next()
            .expect("Perfect square iterator has overflown u64");

        if next > val {
            break;
        }

        if val % next == 0 {
            result = next
        }
    }

    result
}

struct PerfectSquareIter {
    n: u64,
}

impl PerfectSquareIter {
    fn new() -> PerfectSquareIter {
        PerfectSquareIter { n: 0 }
    }
}

impl Iterator for PerfectSquareIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;

        Some(self.n.pow(2))
    }
}
