pub fn nth(n: u32) -> u32 {
    // Building on Azzam's solution
    let mut primes = vec![2, 3];
    while primes.len() <= n as usize {
        // Add next prime we check numbers for addition by incrementing by 2 because even numbers cannot be primes thus last number is odd
        let mut candidate = *primes.last().expect("Starts of with 2 values") + 2;
        while !is_prime(candidate, &primes) {
            candidate += 2;
        }
        primes.push(candidate);
    }
    primes[n as usize] // Cannot just use last because for n == 0 it will not be the last
}

/// Returns true if `candidate` is prime
///
/// Preconditions:
///     - All prime numbers smaller than `candidate` must be in `primes`
fn is_prime(candidate: u32, primes: &[u32]) -> bool {
    primes.iter().all(|x| candidate % x != 0)
}
