// Inverse Property: For each element a in the group, there exists an element b in the group such that a * b = b * a = e, where e is the identity element.

// Exercise 1: Find the inverse of the following element.

const IDENTITY: f32 = 1.0;
// TODO: Find the inverse for the rational number 2.
const INVERSE_OF_TWO: f32 = 1.0 / 2.0;

fn main() {
    let set: Vec<f32> = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
}

fn multiply_operation(a: f32, b: f32) -> f32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_inverse() {
        let a = 2.0;
        let result1 = multiply_operation(a, INVERSE_OF_TWO);
        let result2 = multiply_operation(INVERSE_OF_TWO, a);
        assert_eq!(result1, IDENTITY, "Inverse element is not correct");
        assert_eq!(result2, IDENTITY, "Inverse element is not correct");
    }
}