# Templates & Components

## Templates

To create a new generic circuit in Circom we define a template, which mostly seem like a function if we compare to other common programming languages.

To use a template you need to instantiate it and you can compose larger circuits by using multiple templates. They all need inputs and outputs to communicate with each other. To instantiate it you need to use the `component` keyword and provide the required parameters, which should be known contants at compile time:

```circom
template MyTemplate(a, b) {
    // some code here
}

component circuit = MyTemplate(1, 2)
```

## Components

A component is the instantiation of a template with specific parameters, which creates an arithmetic circuit. Components are immutable and its instantiation can be delayed, since it will be triggered only after all its input signals are assigned to concrete values:

```circom
template MyTemplate(n) {
    signal input x;
    signal input y;
    signal input z;
    signal output out;

    out <== (x + y + z) * n;
}

template Main() {
    signal output out;

    component myComponent = MyTemplate(2);
    myComponent.x <== 1;
    myComponent.y <== 2;
    myComponent.z <== 3; // this will trigger the component instantiation, which means we can only access the output signal after this point

    out <== myComponent.out;
}

component main = Main();
```

## References

- [Templates & Components](https://docs.circom.io/circom-language/templates-and-components/)
- [Circom Workshop by 0xPARC](https://learn.0xparc.org/materials/circom/learning-group-1/circom-1)
