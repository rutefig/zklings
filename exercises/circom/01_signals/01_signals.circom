pragma circom 2.1.9;

template Add() {
    signal input a;
    signal input b;

    // TODO: change the type of signal
    signal input sum;

    sum <== a + b;
}

component main = Add();