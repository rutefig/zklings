# Templates & Components

## Templates

To create a new generic circuit in Circom we define a template, which mostly seem like a function if we compare to other common programming languages.

To use a template you need to instantiate it and you can compose larger circuits by using multiple templates. They all need inputs and outputs to communicate with each other. To instantiate it you need to use the `component` keyword and provide the required parameters:

```circom
template MyTemplate(a, b) {
    // some code here
}

component circuit = MyTemplate(x, y)
```

## Components

## References

- [Templates & Components](https://docs.circom.io/circom-language/templates-and-components/)
- [Circom Workshop by 0xPARC](https://learn.0xparc.org/materials/circom/learning-group-1/circom-1)
