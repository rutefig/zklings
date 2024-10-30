// Pedersen Commitment Zero-Knowledge Proof Exercise

// In this exercise, you'll implement the Pedersen commitment protocol,
// where a prover can commit to a secret value without revealing it,
// and later prove knowledge of that value in a zero-knowledge proof.

// The protocol uses modular arithmetic to simulate group operations.
// In practice, you would use proper elliptic curve operations.

use rand::Rng;

/// Function to compute modular exponentiation
fn mod_pow(base: u128, exponent: u128, modulus: u128) -> u128 {
    // Efficient modular exponentiation
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

/// Parameters for the Pedersen commitment
struct PedersenParams {
    p: u128, // Prime modulus
    g: u128, // Generator g
    h: u128, // Generator h (independent from g)
}

impl PedersenParams {
    fn new() -> Self {
        // For demonstration purposes, we use small prime numbers.
        // In practice, use large primes and secure generators.
        let p = 467u128; // A prime number
        let g = 2u128; // Generator of the group
        let h = 3u128; // Another generator

        // Ensure that g and h are valid generators and that log_g(h) is unknown.
        // In this simplified example, we accept these values.

        PedersenParams { p, g, h }
    }
}

/// Function to compute the Pedersen commitment
fn commit(k: u128, r_k: u128, params: &PedersenParams) -> u128 {
    // K = g^k * h^r_k mod p
    // TODO: Implement the commitment calculation

    // START : Your code here
    unimplemented!()
    // END
}

/// Prover struct
#[allow(non_snake_case)]
struct Prover {
    sk: u128,    // Secret value k
    rho: u128,   // Blinder r_k
    r: u128,     // Random value r (kept secret)
    r_rho: u128, // Random blinder r_rho (kept secret)
    pk: u128,    // Commitment K
    params: PedersenParams,
}

#[allow(non_snake_case)]
impl Prover {
    fn new(sk: u128, rho: u128, params: PedersenParams) -> Self {
        let pk = commit(sk, rho, &params);
        Prover {
            sk,
            rho,
            r: 0,     // Initialized later, for hiding secret key
            r_rho: 0, // Initialized later, for hiding the blinder
            pk,
            params,
        }
    }

    /// Prover's commitment phase (Round 1)
    fn round1(&mut self) -> u128 {
        // Generate random values r and r_rho, compute R = g^r * h^r_rho mod p

        // TODO: Generate random r and r_rho, compute R

        // START: Your code here
        unimplemented!()
        // END

        // R // Return R to the verifier
    }

    /// Prover's response phase (Round 3)
    fn round3(&self, c: u128) -> (u128, u128) {
        // TODO : Compute z and z_rho

        // Compute z = r + c * sk

        // Compute z_rho = r_rho + c * rho

        // (z, z_rho) // Send z and z_rho to the verifier
    }
}

/// Verifier struct
#[allow(non_snake_case)]
struct Verifier {
    pk: u128, // Commitment K
    params: PedersenParams,
}

impl Verifier {
    fn new(pk: u128, params: PedersenParams) -> Self {
        Verifier { pk, params }
    }

    /// Verifier's challenge (Round 2)
    fn round2(&self) -> u128 {
        // TODO : Generate random challenge c
    }

    /// Verifier's verification step
    fn verify(&self, R: u128, c: u128, z: u128, z_rho: u128) -> bool {
        // TODO : Compute left = g^z * h^z_rho mod p

        // TODO : Compute right = R * K^c mod p

        // Check if left == right
    }
}

#[allow(non_snake_case)]
fn main() {
    // Initialize parameters
    let params = PedersenParams::new();

    // Prover's secret values
    let mut rng = rand::thread_rng();
    let sk = rng.gen_range(1..params.p); // Secret value k
    let rho = rng.gen_range(1..params.p); // Blinder rho

    // Initialize Prover
    let mut prover = Prover::new(sk, rho, params);

    // Prover's commitment phase
    let R = prover.round1();

    // Initialize Verifier with Prover's commitment K
    let verifier = Verifier::new(prover.pk, prover.params.clone());

    // Verifier sends challenge c
    let c = verifier.round2();

    // Prover computes responses
    let (z, z_rho) = prover.round3(c);

    // Verifier checks the proof
    let is_valid = verifier.verify(R, c, z, z_rho);

    if is_valid {
        println!(
            "Verification successful. Prover has demonstrated knowledge of the committed value."
        );
    } else {
        println!(
            "Verification failed. Prover could not demonstrate knowledge of the committed value."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn test_pedersen_protocol() {
        // Initialize parameters
        let params = PedersenParams::new();

        // Prover's secret values
        let mut rng = rand::thread_rng();
        let sk = rng.gen_range(1..params.p); // Secret value k
        let rho = rng.gen_range(1..params.p); // Blinder rho

        // Initialize Prover
        let mut prover = Prover::new(sk, rho, params);

        // Prover's commitment phase
        let R = prover.round1();

        // Initialize Verifier with Prover's commitment K
        let verifier = Verifier::new(prover.pk, prover.params.clone());

        // Verifier sends challenge c
        let c = verifier.round2();

        // Prover computes responses
        let (z, z_rho) = prover.round3(c);

        // Verifier checks the proof
        let is_valid = verifier.verify(R, c, z, z_rho);

        assert!(is_valid, "Verification failed in the test case.");
    }
}
