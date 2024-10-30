# Pedersen Commitment Protocol

The basic commitment setup is as follows:

- Prover commits to a secret value `k` with a random blinder `r_k`.
- The commitment `K` is computed as:

```text
K = g^k * h^r_k mod p
```

Here, `g` and `h` are generators in the group.
This commitment can be verified without revealing `k` and `r_k` directly.

## Protocol Steps

1. **Prover's Commitment Phase:**
   - Prover generates random values `r` and `r_rho`, computes `R = g^r * rho mod p`.
2. **Verifier's Challenge:**
   - Verifier sends a random challenge `c` to the prover.
3. **Prover's Response:**
   - Prover computes responses `z = r + c * k` and `z_rho = r_rho + c * r_k`, sends them to the verifier.
4. **Verifier's Verification:**
   - Verifier checks if `g^z * h^z_rho mod p == R * K^c mod p`.
