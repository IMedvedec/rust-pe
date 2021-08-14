/// Function returns the sum of the highest perfect square number
/// that devides each number from 1 to given val.
pub fn sum_of_squares(val: u64) -> u64 {
    let sequence = 1..=val;

    sequence
        .into_iter()
        .map(|x| largest_perfect_square_division(x))
        .sum()
}

/// Function represents the same functionality as sum_of_squares but uses
/// threads for concurrent/parallel execution.
pub fn sum_of_squares_parallel(val: u64) -> u64 {
    use std::sync::mpsc;
    use std::thread;

    let interval = val / 16;

    let seq1 = 1..(8 * interval);
    let seq2 = (8 * interval)..(8 * interval + 4 * interval);
    let seq3 = (12 * interval)..(12 * interval + 3 * interval);
    let seq4 = (15 * interval)..=val;

    let (tx1, rx1): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
    thread::spawn(move || {
        tx1.send(
            seq1.into_iter()
                .map(|x| largest_perfect_square_division(x))
                .sum(),
        )
        .unwrap()
    });

    let (tx2, rx2): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
    thread::spawn(move || {
        tx2.send(
            seq2.into_iter()
                .map(|x| largest_perfect_square_division(x))
                .sum(),
        )
        .unwrap()
    });

    let (tx3, rx3): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
    thread::spawn(move || {
        tx3.send(
            seq3.into_iter()
                .map(|x| largest_perfect_square_division(x))
                .sum(),
        )
        .unwrap()
    });

    let (tx4, rx4): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
    thread::spawn(move || {
        tx4.send(
            seq4.into_iter()
                .map(|x| largest_perfect_square_division(x))
                .sum(),
        )
        .unwrap()
    });

    rx1.recv().unwrap() + rx2.recv().unwrap() + rx3.recv().unwrap() + rx4.recv().unwrap()
}

/// Helper function which returns the largest perfect square number that divies given val.
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

/// Perfect square number iterator.
struct PerfectSquareIter {
    n: u64,
}

impl PerfectSquareIter {
    /// Perfect square number iterator basic constructor.
    fn new() -> PerfectSquareIter {
        PerfectSquareIter { n: 0 }
    }
}

/// Perfect square number iterator implementation.
impl Iterator for PerfectSquareIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;

        Some(self.n.pow(2))
    }
}
