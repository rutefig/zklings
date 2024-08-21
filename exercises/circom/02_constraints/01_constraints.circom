pragma circom 2.1.6;

template Poly() {
    signal input x;
    signal input y;
    signal input z;

    signal output out;

    // TODO: Breakdown this non quadratic expression into multiple quadratic expressions
    out <== x * y * z * 4;
}

component main = Poly();