pub fn nth(n: u32) -> Option<u32> {
    // Okay, time to try this again
    match n {
        // Some easy wins first
        n if n == 0 => None,
        n if n == 1 => Option::from(2),
        n if n == 2 => Option::from(3),
        // Just start going through all the numbers and pull the nth prime. Really inefficient
        n => (1..).filter(|c| is_prime(*c)).nth(n as usize)
    }
}

fn is_prime(candidate: u32) -> bool {
    // Do a basic primality test, as described here https://en.wikipedia.org/wiki/Primality_test
    // I saw someone else do the logic similar to this, and for the longest time I was stumped on
    // why the line started with !. I was looking so hard, assuming it was some kind of weird
    // Rust operator. It finally clicked for me though. It's just boolean logic, nothing special.
    //
    // In a nutshell, it's MUCH more likely that we'll find a divisor for a number than we won't,
    // so the .any is returning true if it finds a divisor that has no remainder. Then it's just
    // bitwise complemented (notted). :facepalm: to me.
    !(2..(candidate as f64).sqrt().floor() as u32 + 1).any(|divisor| candidate % divisor == 0)
}
