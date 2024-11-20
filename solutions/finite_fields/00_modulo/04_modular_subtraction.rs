fn main() {
    let a: i32 = modulo(27 - 14, 10);

    // TODO: Determine the result of the modulo operation.
    assert_eq!(a, 3, "Your result is wrong.");
}

fn modulo(a: i32, m: i32) -> i32 {
    let mut result = a % m;
    if result < 0 {
        result += m;
    }
    result
}