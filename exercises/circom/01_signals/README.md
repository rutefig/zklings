# Simple Addition

This is a simple example of a circuit that adds two numbers and outputs the result.

## Signals

In circom we can define two types of signals: `input` and `output`.
Signals can be assigned with the following operators:

- `<--` and `-->` corresponding to an assignment in the witness generation code produced by the compiler.
- `<==` and `==>` which adds a constraint to the R1CS system stating that the signal is equal to the assigned expression.

Signals can be public or private, and by default in circom, they are private.
Signals are immutable, and once assigned, they cannot be changed, and if this happens, the compiler will throw an error.

## References

[Signals & Variables](https://docs.circom.io/circom-language/signals/)