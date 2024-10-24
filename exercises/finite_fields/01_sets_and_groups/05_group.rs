// Sets and groups

// Exercise 1: Finalise the tests for the identity, inverse and associative properties.

fn main()
{
    let set: Vec<f32> = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let identity_element = 1.0;
    test_for_identity(set[0], identity_element);
    test_for_inverse(set[0], identity_element, identity_element / set[0]);
    test_for_associative(set[0], set[1], set[2]);
}

fn multiply_operation(a: f32, b: f32) -> f32 {
    a * b
}

fn test_for_identity(a: f32, identity: f32) -> bool {
    // TODO: Check for the identity property

}

fn test_for_inverse(a: f32, identity: f32, inverse: f32) -> bool {
    // TODO: Check for the inverse property

}

fn test_for_associative(a: f32, b: f32, c: f32) -> bool {
    // TODO: Check for the associative property

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkers() {
        let set: Vec<f32> = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let identity_element = 1.0;
        assert_eq!(test_for_identity(set[0], identity_element), true, "Identity element is not correct");
        assert_eq!(test_for_inverse(set[0], identity_element, (identity_element / set[0])), true, "Inverse element is not correct");
        assert_eq!(test_for_associative(set[0], set[1], set[2]), true, "Associative property does not hold");
    }
}