use std::thread;

use bitvec::prelude::*;

/// Returns every prime number in the inclusive interval `2..=limit`.
///
/// The interval is split into contiguous segments processed by standard-library
/// worker threads. Each integer in a segment is represented by one bit.
pub fn primes_up_to(limit: u32, requested_threads: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }

    let limit = limit as usize;
    let base_primes = base_primes(integer_sqrt(limit));
    let number_count = limit - 1;
    let worker_count = requested_threads.max(1).min(number_count);
    let segment_size = number_count.div_ceil(worker_count);

    thread::scope(|scope| {
        let mut workers = Vec::with_capacity(worker_count);

        for segment_index in 0..worker_count {
            let low = 2 + segment_index * segment_size;
            if low > limit {
                break;
            }

            let high = low.saturating_add(segment_size - 1).min(limit);
            let base_primes = &base_primes;
            workers.push(scope.spawn(move || sieve_segment(low, high, base_primes)));
        }

        workers
            .into_iter()
            .flat_map(|worker| worker.join().expect("prime worker panicked"))
            .collect()
    })
}

fn sieve_segment(low: usize, high: usize, base_primes: &[usize]) -> Vec<u32> {
    let mut is_prime = bitvec![1; high - low + 1];

    for &prime in base_primes {
        let first_at_or_after_low = low + (prime - low % prime) % prime;
        let mut multiple = (prime * prime).max(first_at_or_after_low);

        while multiple <= high {
            is_prime.set(multiple - low, false);
            multiple += prime;
        }
    }

    is_prime
        .iter_ones()
        .map(|index| (low + index) as u32)
        .collect()
}

fn base_primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    let mut is_prime = bitvec![1; limit + 1];
    is_prime.set(0, false);
    is_prime.set(1, false);

    let mut prime = 2;
    while prime <= limit / prime {
        if is_prime[prime] {
            let mut multiple = prime * prime;
            while multiple <= limit {
                is_prime.set(multiple, false);
                multiple += prime;
            }
        }
        prime += 1;
    }

    is_prime.iter_ones().collect()
}

fn integer_sqrt(number: usize) -> usize {
    if number < 2 {
        return number;
    }

    let mut root = (number as f64).sqrt() as usize;

    while root > number / root {
        root -= 1;
    }
    while root < number / (root + 1) {
        root += 1;
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_limits_below_two() {
        assert!(primes_up_to(0, 4).is_empty());
        assert!(primes_up_to(1, 4).is_empty());
    }

    #[test]
    fn includes_the_upper_bound_when_it_is_prime() {
        assert_eq!(primes_up_to(2, 4), vec![2]);
        assert_eq!(primes_up_to(3, 4), vec![2, 3]);
    }

    #[test]
    fn returns_primes_in_ascending_order() {
        assert_eq!(
            primes_up_to(30, 4),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn gives_the_same_result_for_different_thread_counts() {
        let expected = primes_up_to(10_000, 1);

        for thread_count in [0, 2, 3, 4, 7, 16] {
            assert_eq!(primes_up_to(10_000, thread_count), expected);
        }
    }

    #[test]
    fn finds_the_known_number_of_primes_below_one_million() {
        let primes = primes_up_to(1_000_000, 4);

        assert_eq!(primes.len(), 78_498);
        assert_eq!(primes.last(), Some(&999_983));
    }

    #[test]
    fn integer_square_root_is_exact_around_square_boundaries() {
        assert_eq!(integer_sqrt(15), 3);
        assert_eq!(integer_sqrt(16), 4);
        assert_eq!(integer_sqrt(17), 4);

        let root = integer_sqrt(u32::MAX as usize);
        assert!(root <= u32::MAX as usize / root);
        assert!(root + 1 > u32::MAX as usize / (root + 1));
    }
}
