use std::time::{SystemTime, UNIX_EPOCH};

const P: u64 = 17; // modulus
const G_1: u64 = 5; // generator 1
const G_2: u64 = 7; // generator 2

fn pseudo_random(max: u64) -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let nanos = since_epoch.as_secs() * 1_000_000_000 + since_epoch.subsec_nanos() as u64;
    nanos % max + 1
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn pseudo_random_large_prime(max: u64) -> u64 {
    let mut r = pseudo_random(max);
    while !is_prime(r) || r <= 10 {
        r = pseudo_random(max);
    }
    r
}

fn commit(a_1: u64, a_2: u64) -> u64 {
    (mod_exp(G_1, a_1, P) * mod_exp(G_2, a_2, P)) % P
}

fn extended_gcd(a: u64, b: u64) -> (u64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a as i64 / b as i64) * y)
    }
}

fn mod_inverse(x: u64) -> Option<u64> {
    let (g, inv_x, _) = extended_gcd(x, P - 1);
    if g != 1 {
        None
    } else {
        Some(((inv_x % (P - 1) as i64 + (P - 1) as i64) % (P - 1) as i64) as u64)
    }
}

fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

struct VerificationParams {
    z: u64,
    commitment_r: u64,
    commitment_a: u64,
    c_l: u64,
    c_r: u64,
    inverse_mu: u64,
    mu: u64,
    challenge: u64,
}

fn verify(params: VerificationParams) -> bool {
    let g = G_1 * mod_exp(G_2, params.inverse_mu, P);
    mod_exp(g, params.z, P)
        == params.commitment_r
            * mod_exp(params.commitment_a, params.challenge, P)
            * mod_exp(params.c_l, params.inverse_mu, P)
            * mod_exp(params.c_r, params.mu, P)
            % P
}

fn run_split_and_fold(a_1: u64, a_2: u64) -> bool {
    let commitment_a = commit(a_1, a_2);

    // prover - round 1
    let r_1 = pseudo_random(P);
    let r_2 = pseudo_random(P);
    let commitment_r = commit(r_1, r_2);

    // verifier - round 2
    let challenge = pseudo_random(P);

    // prover - round 3
    let z_1 = r_1 + challenge * a_1;
    let z_2 = r_2 + challenge * a_2;

    let c_l = mod_exp(G_2, z_1, P);
    let c_r = mod_exp(G_1, z_2, P);

    // verifier - round 4
    let mu = pseudo_random_large_prime(P);
    let inverse_mu = mod_inverse(mu).unwrap();

    // prover - round 5
    let z = z_1 + mu * z_2;

    // verify
    verify(VerificationParams {
        z,
        commitment_r,
        commitment_a,
        c_l,
        c_r,
        inverse_mu,
        mu,
        challenge,
    })
}

fn main() {
    let is_valid = run_split_and_fold(13, 19);
    println!("Is proof valid? {}", is_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_fold() {
        assert!(run_split_and_fold(13, 19));
        assert!(run_split_and_fold(11, 31));
        assert!(run_split_and_fold(29, 47));
    }
}
