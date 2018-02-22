pub fn nth(n: usize) -> Result< usize, &'static str> {
    if n < 1 {
        return Err("Input must be superior to 0.");
    }
    let mut primes = vec!(2);
    while primes.len() < n {
        // Bertrand theroem: for a given number x, there's
        // always a prime p such that x < p < 2x
        let last_prime = primes[primes.len() - 1];
        let next = last_prime + 1;
        
        for number in next..2*next {
            if is_prime(number, &primes) {
                primes.push(number);
            }
        }
    }
    Ok(primes[n - 1])
}

pub fn is_prime(i: usize, primes: &Vec<usize>) -> bool {
    for prime in primes{
        if i % prime == 0 {
            return false;
        }
    }
    true
}