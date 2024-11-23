## Introduction

The **Pedersen Commitment** is a cryptographic protocol that allows a prover to commit to a chosen value while keeping it hidden from others, with the ability to reveal or prove knowledge of the committed value later. It is a fundamental building block in cryptography, particularly in zero-knowledge proofs, secure multiparty computation, and blockchain technologies.


## Mathematical Background

Before diving into the Pedersen commitment scheme, it's essential to understand some mathematical concepts:

- **Modular Arithmetic:** Arithmetic operations performed with respect to a modulus $n$, where numbers "wrap around" upon reaching $n$.
- **Finite Fields:** A set of integers ${0, ..., p-1}$, with $p$ elements where $p$ is a prime number and we have defined the addition and multiplication operations to be modulo $p$.
- **Discrete Logarithm Problem:** Given $g$ and $y$, find $x$ such that $g^x=y\ mod \ p$. This problem is computationally hard for large primes, which provides security in cryptographic protocols.
- **Generator of a Mulitplicative Group:** An element that can produce every other element in the group through repeated multiplication. In a finite field’s multiplicative group, if an element $g$ is a generator, then powers of $g$ will sequentially yield all non-zero elements in the group.


## Pedersen Commitment Scheme

The Pedersen commitment scheme allows a prover to commit to a secret value without revealing it and later prove knowledge of that value. It relies on the hardness of the discrete logarithm problem for its security.

**Setup Parameters:**

- $p$: A large prime number (modulus).
- $G$: A cyclic group of order $q$ (a large prime).
- $g, h$: Generators of the group $G$, such that the discrete logarithm $\log_g h$ is unknown.

**Commitment Computation:**

To commit to a secret value $k$:

1. The prover selects a random blinding factor $r_k$ from $\mathbb{Z}_q$.
2. Computes the commitment $K$:

   $K = g^k \cdot h^{r_k} \mod p$

## Protocol Steps

The Pedersen commitment can be used in a zero-knowledge proof where the prover demonstrates knowledge of the committed value $k$ without revealing it.

**Protocol Overview:**

1. **Prover's Commitment Phase:**

   - Prover generates random values $r$ and $r_{\rho}$ from $\mathbb{Z}_q$.
   - Computes:
     $R = g^r \cdot h^{r_{\rho}} \mod p$

   - Sends $R$ to the verifier.

2. **Verifier's Challenge:**

   - Verifier selects a random challenge $c$ from $\mathbb{Z}_q$.
   - Sends $c$ to the prover.

3. **Prover's Response:**

   - Prover computes:

     $z = r + c \cdot k$<br>
     $z_{\rho} = r_{\rho} + c \cdot r_k$

   - Sends $z$ and $z_{\rho}$ to the verifier.

4. **Verifier's Verification:**

   - Verifier checks whether:
     $g^z \cdot h^{z_{\rho}} \mod p \stackrel{?}{=} R \cdot K^c \mod p
     $
   - If the equality holds, the verifier is convinced that the prover knows $k$.

