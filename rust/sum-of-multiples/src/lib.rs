pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| is_multiple_of_factors(x, factors)).sum()
}

pub fn is_multiple_of_factors(n: &u32, factors: &[u32]) -> bool{
    factors.iter().fold(false, |acc, x| acc || (n % x == 0))
}
