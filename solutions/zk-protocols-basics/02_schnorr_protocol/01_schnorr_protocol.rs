// Schnorr Protocol Exercise

use rand::Rng;

// Function to compute modular exponentiation
fn mod_pow(base: u128, exponent: u128, modulus: u128) -> u128 {
    let mut result = 1u128;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exponent /= 2;
    }
    result
}

// Struct representing the Schnorr protocol
struct SchnorrProtocol {
    p: u128, // Large prime
    g: u128, // Generator
    x: u128, // Private key
    y: u128, // Public key (g^x mod p)
}

impl SchnorrProtocol {
    fn new() -> Self {
        // For demonstration, we use small primes. In practice, use large primes.
        let p = 467u128; // p is a prime
        let g = 2u128;   // g is a generator modulo p

        // Prover's private key x (random number less than p - 1)
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..p - 1); // x ∈ [1, p - 2]

        // Compute public key y = g^x mod p
        let y = mod_pow(g, x, p);

        SchnorrProtocol { p, g, x, y }
    }

    // Prover's commitment step
    fn prover_commitment(&self) -> (u128, u128) {
        // Prover selects a random nonce r ∈ [1, p - 2]
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.p - 1);

        // Compute t = g^r mod p
        let t = mod_pow(self.g, r, self.p);

        // Return t and r
        (t, r)
    }

    // Prover's response step
    fn prover_response(&self, r: u128, c: u128) -> u128 {
        // Compute s = (r + c * x) mod (p - 1)
        let s = r + c * self.x;
        s
    }

    // Verifier's verification step
    fn verifier_check(&self, t: u128, s: u128, c: u128) -> bool {
        // Compute left = g^s mod p
        let left = mod_pow(self.g, s, self.p);

        // Compute right = t * y^c mod p
        let y_c = mod_pow(self.y, c, self.p);
        let right = (t * y_c) % self.p;

        // Check if left == right
        left == right
    }
}

fn main() {
    let protocol = SchnorrProtocol::new();

    // Prover's commitment
    let (t, r) = protocol.prover_commitment();

    // Verifier generates a random challenge c ∈ [1, p - 2]
    let mut rng = rand::thread_rng();
    let c = rng.gen_range(1..protocol.p - 1);

    // Prover's response
    let s = protocol.prover_response(r, c);

    // Verifier checks the proof
    let is_valid = protocol.verifier_check(t, s, c);

    if is_valid {
        println!("Verification successful. Prover has demonstrated knowledge of the private key.");
    } else {
        println!("Verification failed. Prover could not demonstrate knowledge of the private key.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schnorr_protocol() {
        let protocol = SchnorrProtocol::new();

        // Prover's commitment
        let (t, r) = protocol.prover_commitment();

        // Verifier generates a random challenge c ∈ [1, p - 2]
        let mut rng = rand::thread_rng();
        let c = rng.gen_range(1..protocol.p - 1);

        // Prover's response
        let s = protocol.prover_response(r, c);

        // Verifier checks the proof
        let is_valid = protocol.verifier_check(t, s, c);

        assert!(is_valid, "Verification failed in the test case.");
    }
}
