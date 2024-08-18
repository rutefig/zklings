# Circom

Circom is a domain-specific language for writing circuits for zkSNARKS. For writing a circuit in Circom, you need to define the signals and constraints that need to be satisfied to generate a valid proof.

When programming in Circom, we need to think that we are defining two sides of the protocol. The first side is the witness generation code, which is the code that will be executed by the prover to generate the witness, and the second side is the constraint system, which is the code that will be executed by the verifier to verify the proof.
