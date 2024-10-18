// Sets and groups

// To better understand finite fields, and what fields are in general, we need to start with elements, sets, groups and operations.

// Exercise 1: Implement a simple addition operation on a set of integers.

use std::vec::Vec;

fn main() {
    // Given a set of integers. Each integer number is an element of the set.
    let set: Vec<i32> = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];

    // We can run operations any set element.
    let result = add_operation(set[1], set[2]);

    // print result
    println!("Result of addition: {}", result);
}

// TODO: Implement this function
fn add_operation(a: i32, b: i32) -> i32 {

}

// Learning: A group is the combination of a set and a binary operation.