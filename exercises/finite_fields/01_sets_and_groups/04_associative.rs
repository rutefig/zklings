// Sets and groups

// Associative Property: For all elements a, b, and c in the group, (a * b) * c = a * (b * c).

// Exercise 1: Prove that the set has the associative property.

fn main() {
    let set: Vec<f32> = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    
    // TODO: pass the test.
    let left_side = multiply_operation(multiply_operation(XXX, XXX), XXX);
    let right_side = multiply_operation(XXX, multiply_operation(XXX, XXX));
    assert!(left_side == right_side, "associative property doesnt hold");
}

fn multiply_operation(a: f32, b: f32) -> f32 {
    a * b
}