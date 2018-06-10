extern crate primes;
use primes::PrimeSet;

pub fn nth(n: u32) -> Option<u32> {
    // Go stumped, caved, and used a library. Just submitting so I can see others' solutions.
    if n == 0 { return Option::None; }

    let mut pset = PrimeSet::new();
    let mut found_prime: Option<u32> = Option::None;
    for (_, n) in pset.iter().enumerate().skip((n - 1) as usize).take(1) {
        found_prime = Some(n as u32);
    }

    return found_prime;
}
