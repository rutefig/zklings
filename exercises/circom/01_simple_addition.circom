pragma circom 2.1.9;

template Add() {
    signal input a;
    signal input b;

    signal output sum;

    sum <== a + b;
}

component main = Add();