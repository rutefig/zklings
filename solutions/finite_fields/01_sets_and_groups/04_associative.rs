// Associative Property: For all elements a, b, and c in the group, (a * b) * c = a * (b * c).

// Exercise 1: Prove that the set has the associative property.

fn main() {
    let set: Vec<f32> = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    
    // TODO: insert the correct values.
    let left_side = multiply_operation(multiply_operation(set[1], set[2]), set[3]);
    let right_side = multiply_operation(set[1], multiply_operation(set[2], set[3]));
    assert!(left_side == right_side, "associative property doesnt hold");
}

fn multiply_operation(a: f32, b: f32) -> f32 {
    a * b
}