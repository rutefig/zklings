// Note: The modulo operator differs from the % (i.e. remainder) operator, where modulo always returns a non-negative result.
// In the syntax below, modulo(7, 4) equals to 7 mod 4.

fn main() {
    let a: i32 = modulo(7, 4);

    // TODO: Determine the result of the modulo operation.
    assert_eq!(a, XXX, "Your result is wrong.");
}

// No need to change anything in the code below.
fn modulo(a: i32, m: i32) -> i32 {
    let mut result = a % m;
    if result < 0 {
        result += m;
    }
    result
}