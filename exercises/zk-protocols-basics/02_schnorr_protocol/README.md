# Schnorr Protocol 

The **Schnorr protocol** is a cryptographic protocol that allows a prover to demonstrate knowledge of a secret (private key) without revealing any information about it. It's a type of **zero-knowledge proof** and forms the basis for various digital signature schemes, including the Schnorr signature algorithm.

Understanding the Schnorr protocol is essential for grasping modern cryptographic techniques used in secure communications, authentication systems, and blockchain technologies.

---

## Mathematical Background

Before diving into the protocol, it's crucial to understand some mathematical concepts:

- **Modular Arithmetic:** Arithmetic operations performed with respect to a modulus $n$, where numbers "wrap around" upon reaching $n$.
- **Finite Fields:** A set of numbers with addition, subtraction, multiplication, and division operations defined, satisfying certain properties.
- **Discrete Logarithm Problem:** Given $g$ and $y$, find $x$ such that $g^x=y\ mod p$. This problem is computationally hard for large primes, which provides security in cryptographic protocols.

**Key Parameters:**

- $p$: A large prime number.
- $g$: A generator of the multiplicative group, meaning $g$ is a primitive root modulo $p$.
- $x$: The prover's private key, a secret number.
- $y$: The prover's public key, computed as  $y$ = $g^x\ mod p $ 


## Schnorr Protocol Steps

The protocol involves two parties:

- **Prover:** Wants to prove knowledge of the secret $x$ without revealing it.
- **Verifier:** Wants to verify that the prover knows $x$.

**Protocol Steps:**

1. **Prover's Commitment:**

   - Prover selects a random nonce $r$ from $[1, p - 2]$.
   - Computes the commitment $t = g^r\ mod p$.
   - Sends $t$ to the verifier.

2. **Verifier's Challenge:**

   - Verifier selects a random challenge $c$ from $[1, p - 2]$.
   - Sends $c$ to the prover.

3. **Prover's Response:**

   - Prover computes the response $s = r + c \cdot x$.
   - Sends $s$ to the verifier.

4. **Verifier's Verification:**

   - Verifier computes:
     - $\text{left} = g^s \mod p$.
     - $\text{right} = t \cdot y^c \mod p$.
   - Verifier checks if $\text{left} \equiv \text{right} \mod p$.
     - If equal, the proof is accepted.
     - If not, the proof is rejected.

---

## Detailed Example

Let's walk through an example with small numbers to illustrate each step.

**Parameters:**

- **Prime $p$:** 23
- **Generator $g$:** 5 (a primitive root modulo 23)
- **Private Key $x$:** 6 (chosen by the prover)

### Step 0: Setup

- **Compute Public Key $y$:**

  
  $y = g^x \mod p = 5^6 \mod 23 = 15625 \mod 23 = 8$
  

### Step 1: Prover's Commitment

- **Select Random Nonce $r$:**

  Let's choose $r = 15$.

- **Compute Commitment $t$:**

  $t = g^r \mod p = 5^{15} \mod 23$


  Calculating $5^{15} \mod 23$:

  - Use modular exponentiation techniques or software to compute $t = 19$.

- **Prover Sends $t = 19$ to Verifier.**

### Step 2: Verifier's Challenge

- **Select Random Challenge $c$:**

  Let's choose $c = 7$.

- **Verifier Sends $c = 7$ to Prover.**

### Step 3: Prover's Response

- **Compute Response $s$:**

  
  $s = r + c \cdot x = 15 + 7 \times 6 = 15 + 42 = 57$
  

- **Prover Sends $s = 57$ to Verifier.**

### Step 4: Verifier's Verificati$
- **Compute Left Side $g^s \mod p$:**

  
  $\text{left} = 5^{57} \mod 23$


  - Compute $\text{left}$ using modular exponentiation to get $\text{left} = 21$.

- **Compute Right Side $t \cdot y^c \mod p$:**
  
  $\text{right} = t \cdot y^c \mod p = 19 \cdot 8^7 \mod 23$


  - Compute $y^7 \mod 23$ and then $\text{right}$ to get $\text{right} = 21$.

- **Verification:**


  $\text{left} = 21, \quad \text{right} = 21$


  Since $\text{left} = \text{right}$, the verifier accepts the proof.




