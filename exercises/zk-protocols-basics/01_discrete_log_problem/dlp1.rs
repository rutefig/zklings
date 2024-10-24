// Discrete Logarithm Exercise

// The Discrete Logarithm Problem (DLP) is the problem of finding the exponent x in the equation:
// a^x ≡ b mod n, where a, b, and n are given integers, and n is a prime number.

// For small values of n, we can solve this problem using a simple brute-force search.
// However, for large values of n, the problem becomes computationally infeasible, which is the basis
// of security for many cryptographic systems.

// In this exercise, you'll implement a function to solve the discrete logarithm problem for small primes.

fn mod_pow(a: u64, x: u64, n: u64) -> u64 {
    // Computes (a^x) mod n using modular exponentiation.
    let mut result = 1;
    let mut base = a % n;
    let mut exponent = x;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % n;
        }
        base = (base * base) % n;
        exponent /= 2;
    }
    result
}

// TODO: Implement this function
fn discrete_log(a: u64, b: u64, n: u64) -> Option<u64> {
    // Finds the smallest non-negative integer x such that a^x ≡ b mod n
    // Returns Some(x) if such x exists, or None if no solution exists.
    // For small n, a simple brute-force search is feasible.

    // Hint: Try all possible x from 0 to n-1 and check if mod_pow(a, x, n) == b

    // START: Implement the function here
    
    // END
}

fn main() {
    let a = 5;
    let b = 8;
    let n = 23;

    match discrete_log(a, b, n) {
        Some(x) => println!("The discrete log of {} base {} modulo {} is {}", b, a, n, x),
        None => println!("No solution found."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discrete_log() {
        assert_eq!(discrete_log(2, 8, 13), Some(3)); // 2^3 ≡ 8 mod 13
        assert_eq!(discrete_log(5, 8, 23), Some(6)); // 5^6 ≡ 8 mod 23
        assert_eq!(discrete_log(7, 11, 19), Some(2)); // 7^2 ≡ 11 mod 19
    }
}
