// Multiple Solutions in Discrete Logarithm Problem

// In this exercise, you'll implement a function to find more than one exponents x such that a^x ≡ b mod n.
// This will help illustrate that, in certain cases, there are multiple solutions to the discrete logarithm problem,
// making it difficult to determine which exponent was originally used.

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
fn find_at_least_5_exponents(a: u64, b: u64, n: u64) -> Vec<u64> {
    // Finds at least 5 non-negative integers x such that a^x ≡ b mod n within the range 0 <= x < n-1
    // Returns a vector containing all such x values.

    // START: Implement the function here
    let mut exponents = Vec::new();
    let upper_limit = n * 5; // You can adjust the range as needed
    for x in 0..upper_limit {
        if mod_pow(a, x, n) == b % n {
            exponents.push(x);
            if exponents.len() >=5 {
                break;
            }
        }
    }
    exponents
    
    // END
}

fn main() {
    let a = 4;
    let b = 1;
    let n = 15;

    let exponents = find_at_least_5_exponents(a, b, n);
    if !exponents.is_empty() {
        println!("Exponents x such that {}^x ≡ {} mod {} are:", a, b, n);
        for x in exponents {
            println!("{}", x);
        }
    } else {
        println!("No solutions found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_at_least_5_exponents() {
        // For a = 4, b = 1, n = 15, the exponents satisfying 4^x ≡ 1 mod 15 are x = 0, 4, 8, 12
        assert_eq!(find_at_least_5_exponents(4, 1, 15), vec![0, 2, 4, 6, 8]);

        // For a = 2, b = 1, n = 5, exponents are x = 0, 2, 4, 6, 8
        assert_eq!(find_at_least_5_exponents(2, 1, 5), vec![0, 4, 8 ,12 ,16]);

        // For a = 7, b = 7, n = 13, exponents are x = 1, 13, 25, 37, 49
        assert_eq!(find_at_least_5_exponents(7, 7, 13), vec![1, 13, 25, 37, 49]);

        // No solutions
        assert_eq!(find_at_least_5_exponents(5, 3, 13), vec![]);
    }
}
