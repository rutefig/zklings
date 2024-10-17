# Discrete Logarithm Problem (DLP)

## Overview

The **Discrete Logarithm Problem (DLP)** is a fundamental problem in the field of cryptography. It involves finding the exponent \( x \) in the equation:<br/>
$\alpha^x$=$\beta$ mod p, where `p` is the prime modulus.

We call that `x` is the discrete logarithm of $\beta$ with respect to base $\alpha$ and denote it with $\log_\alpha \beta$.


The DLP is computationally hard for large values of  p  and  x , making it useful in cryptographic algorithms such as Diffie-Hellman key exchange, ElGamal, and DSA.

Let's consider an example with small numbers.

<!-- > Suppose G = F<sub>101</sub><sup>x</sup>. Then log3 37 = 24, since 3<sup>24</sup> ≡ 37 mod 101. -->

Let’s consider an example with small numbers:

g = 2 ,<br>
h = 3 ,<br>
p = 5 .<br>

We need to find  x  such that:

2<sup>x</sup> = 3 mod 5

Solution:

We try different values of  x :

 2<sup>1</sup> = 2 ,<br>
 2<sup>2</sup> = 4 ,<br>
 2<sup>3</sup> = 8 mod 5 = 3<br>

Thus,  x = 3  is the solution.