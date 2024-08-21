pragma circom 2.1.6;

template A(x) {
    signal input in;
    signal output out;

    out <== x * in;
}

template Main() {
    signal input a;

    signal output out;

    // TODO: Instantiate the template A
}

component main = Main();