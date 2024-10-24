// Identity Property: There exists an element e in the group such that for every element a in the group, e * a = a * e = a.

// Exercise 1: Find the identity of the following set.

// TODO: replace XXX with the identity element.
const IDENTITY: f32 = 1.0;

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
    fn test_for_identity() {
        let a = 2.0;
        let result1 = multiply_operation(a, IDENTITY);
        let result2 = multiply_operation(IDENTITY, a);
        assert_eq!(result1, a, "Identity element is not correct");
        assert_eq!(result2, a, "Identity element is not correct");
    }
}