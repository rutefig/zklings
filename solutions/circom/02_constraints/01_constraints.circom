pragma circom 2.1.6;

template Poly() {
    signal input x;
    signal input y;
    signal input z;

    signal output out;

    // out <== x * y * z * 4;
    signal tmp1;
    signal tmp2;
    tmp1 <== x * y;
    tmp2 <== tmp1 * z;
    out <== tmp2 * 4;
}

component main = Poly();