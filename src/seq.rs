use bitvec::prelude::*;

/// Returns every prime number in the inclusive interval `2..=limit`.
pub fn primes_up_to(limit: u32) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }

    let limit = limit as usize;
    let bit_count = limit
        .checked_add(1)
        .expect("the maximum u32 limit requires a 64-bit target");
    let mut is_prime = bitvec![1; bit_count];
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

    is_prime.iter_ones().map(|prime| prime as u32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_small_limits() {
        assert!(primes_up_to(0).is_empty());
        assert!(primes_up_to(1).is_empty());
        assert_eq!(primes_up_to(2), vec![2]);
    }

    #[test]
    fn returns_primes_in_ascending_order() {
        assert_eq!(primes_up_to(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn finds_the_known_number_of_primes_below_one_million() {
        let primes = primes_up_to(1_000_000);

        assert_eq!(primes.len(), 78_498);
        assert_eq!(primes.last(), Some(&999_983));
    }
}
