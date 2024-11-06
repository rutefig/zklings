# Modular Arithmetic

## What is Modular Arithmetic?

Modular arithmetic is a system of arithmetic for integers, where numbers "wrap around" when reaching a certain value, called the modulus. It's like a clock, where after 12, we start over at 1.

In mathematical notation, we write:

a ≡ b (mod m)

This is read as "a is congruent to b modulo m" and means that m divides the difference of a and b; or in other words, a and b have the same remainder when divided by m.

For example:

- 38 ≡ 14 (mod 12) because 38 - 14 = 24, which is divisible by 12
- In a clock system, 3 o'clock plus 10 hours is 1 o'clock, or 3 + 10 ≡ 1 (mod 12)

## Properties of Modular Arithmetic

1. **Closure**: The result of any operation in modular arithmetic is always another integer within the given modulus.

```math
    Example: (7 + 8) mod 10 = 5
```

2. **Associativity**: The grouping of operations doesn't matter.

```math
   (a + b) + c ≡ a + (b + c) (mod m)
   (a * b) * c ≡ a * (b * c) (mod m)
```

3. **Commutativity**: The order of operands doesn't matter for addition and multiplication.

```math
   a + b ≡ b + a (mod m)
   a * b ≡ b * a (mod m)
```

4. **Distributivity**: Multiplication distributes over addition.

```math
   a * (b + c) ≡ (a * b) + (a * c) (mod m)
```

5. **Identity**: 0 is the additive identity, 1 is the multiplicative identity.

```math
   a + 0 ≡ a (mod m)
   a * 1 ≡ a (mod m)
```

6. **Additive Inverse**: For every a, there exists an additive inverse -a such that:

```math
   a + (-a) ≡ 0 (mod m)
```

7. **Multiplicative Inverse**: If a and m are coprime, there exists a multiplicative inverse a^(-1) such that:

```math
   a * a^(-1) ≡ 1 (mod m)
```

Modular arithmetic is fundamental to many areas of computer science and cryptography, including error-correcting codes, hash functions, and encryption algorithms. Understanding these properties is crucial for working with Zero-Knowledge Proofs and other cryptographic protocols.