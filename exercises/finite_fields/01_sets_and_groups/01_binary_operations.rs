// Sets and groups

// A group is the combination of a set and a binary operation.

// What is a binary operation? A binary operation can be understood as a function f (a, b) that applies two elements of the same set S, such that the result will also be an element of the set S.

// Exercise 1: The following are different operations on different sets. Identify the non-binary operations and fix them.

use std::vec::Vec;
use std::any::type_name;
use std::any::Any;

fn main() {
    // Set of integers
    let _set_integers: Vec<i32> = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];

    // Set of strings
    let _set_strings: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    // Set of floating-point numbers
    let _set_floats: Vec<f64> = vec![0.1, 0.2, 0.3, 0.4, 0.5];
}


// TODO: Find the non-binary operations and fix it.
fn add_operation(a: i32, b: f64) -> f64 {
    (a as f64) + b
}

fn multiply_operation(a: f64, b: f64) -> f64 {
    a * b
}

fn concat_operation(a: &str, b: i32) -> String {
    format!("{}{}", a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_of<T: ?Sized + Any>(_: &T) -> String {
        type_name::<T>().to_string()
    }

    #[test]
    fn test_add_operation_inputs() {
        let a: i32 = 5;
        let b: i32 = 3;
        assert_eq!(type_of(&a), type_of(&b), "Input types for add_operation should be the same");
        add_operation(a, b); // Just to ensure the function is called
    }

    #[test]
    fn test_multiply_operation_inputs() {
        let a: f64 = 0.5;
        let b: f64 = 0.3;
        assert_eq!(type_of(&a), type_of(&b), "Input types for multiply_operation should be the same");
        multiply_operation(a, b); // Just to ensure the function is called
    }

    #[test]
    fn test_concat_operation_inputs() {
        let a: str = "hello";
        let b: str = "world";
        assert_eq!(type_of(&a), type_of(&b), "Input types for concat_operation should be the same");
        concat_operation(a, b); // Just to ensure the function is called
    }
}